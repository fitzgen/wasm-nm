/*!

[![](https://docs.rs/wasm-nm/badge.svg)](https://docs.rs/wasm-nm/) [![](https://img.shields.io/crates/v/wasm-nm.svg)](https://crates.io/crates/wasm-nm) [![](https://img.shields.io/crates/d/wasm-nm.png)](https://crates.io/crates/wasm-nm) [![Build Status](https://travis-ci.org/fitzgen/wasm-nm.png?branch=master)](https://travis-ci.org/fitzgen/wasm-nm)

List the symbols within a wasm file.

* [Library](#library)
* [Executable](#executable)
* [License](#license)
* [Contributing](#contributing)

## Executable

To install the `wasm-nm` executable, run

```text
$ cargo install wasm-nm
```

For information on using the `wasm-nm` executable, run

```text
$ wasm-nm --help
```

### Using `wasm-nm` as a Size Profiler

`wasm-nm` can function as a rudimentary size profiler for `.wasm` files.

The `-z` option enables printing a function's code size. The unix `sort` utility
can be used to sort the symbols by size. The `rustfilt` utility can be used to
demangle Rust symbols (`cargo install rustfilt`).

```text
$ wasm-nm -z path/to/something.wasm | sort -n -u -r | rustfilt | head
3578 p dlmalloc::dlmalloc::Dlmalloc::malloc::hb37c2fafc9847520
3307 e quicksilver
1427 p <str as core::fmt::Debug>::fmt::h0cf4ea19d7121472
1287 p std::panicking::rust_panic_with_hook::h52b2005910c55f47
1268 p core::fmt::Formatter::pad::hdb2be9f507201bd1
1248 p core::str::slice_error_fail::h09ffe3974e261c49
1064 p core::fmt::write::h914fcaafc6fb200a
987 p core::fmt::Formatter::pad_integral::h2f2f83d99c318b28
945 p <&'a T as core::fmt::Debug>::fmt::h4a5a01d440d30f67
918 p dlmalloc::dlmalloc::Dlmalloc::free::h8185738df2a87b48
```

## Library

To use `wasm-nm` as a library, add this to your `Cargo.toml`:

```toml
[dependencies.wasm-nm]
# Do not build the executable.
default-features = false
```

See [docs.rs/wasm-nm][docs] for API documentation.

[docs]: https://docs.rs/wasm-nm

## License

Licensed under either of

 * [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)

 * [MIT license](http://opensource.org/licenses/MIT)

at your option.

## Contributing

See
[CONTRIBUTING.md](https://github.com/fitzgen/wasm-nm/blob/master/CONTRIBUTING.md)
for hacking.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

 */

#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

extern crate failure;
extern crate parity_wasm;

use parity_wasm::elements::{Deserialize, FuncBody, ImportEntry, Internal, Module, Section,
                            Serialize, VarUint32, VarUint7};
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use std::iter;
use std::io;
use std::slice;
use std::str;

/// Options for controlling which symbols are iterated over.
#[derive(Clone, Debug)]
pub struct Options {
    /// Should imported symbols be iterated over?
    pub imports: bool,

    /// Should exported symbols be iterated over?
    pub exports: bool,

    /// Should private symbols be iterated over?
    pub privates: bool,

    /// Should the symbols' sizes be computed?
    pub sizes: bool,
}

impl Default for Options {
    fn default() -> Options {
        Options {
            imports: true,
            exports: true,
            privates: true,
            sizes: false,
        }
    }
}

impl Options {
    /// Construct options for iterating over *none* of the symbol kinds.
    pub fn nothing() -> Options {
        Options {
            imports: false,
            exports: false,
            privates: false,
            sizes: false,
        }
    }
}

/// Get the symbols in the given wasm file.
pub fn symbols<R>(opts: Options, reader: &mut R) -> Result<Symbols, failure::Error>
where
    R: io::Read,
{
    let module = Module::deserialize(reader)?;
    Ok(Symbols { opts, module })
}

/// The set of symbols in a wasm file.
pub struct Symbols {
    opts: Options,
    module: Module,
}

impl fmt::Debug for Symbols {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Symbols")
            .field("opts", &self.opts)
            .field("module", &"...")
            .finish()
    }
}

// Cribbed from wasm-gc; waiting for the name section support to be upstreamed
// into parity-wasm.
fn decode_name_map<'a>(
    mut bytes: &'a [u8],
    num_imports: usize,
) -> Result<HashMap<u32, Cow<'a, str>>, failure::Error> {
    while !bytes.is_empty() {
        let name_type = u8::from(VarUint7::deserialize(&mut bytes)?);
        let name_payload_len = u32::from(VarUint32::deserialize(&mut bytes)?);
        let (these_bytes, rest) = bytes.split_at(name_payload_len as usize);

        if name_type == 1 {
            bytes = these_bytes;
        } else {
            bytes = rest;
            continue;
        }

        let count = u32::from(VarUint32::deserialize(&mut bytes)?);
        let mut names = HashMap::with_capacity(count as usize);
        for _ in 0..count {
            let index =
                u32::from(VarUint32::deserialize(&mut bytes)?).saturating_sub(num_imports as u32);
            let name_len = u32::from(VarUint32::deserialize(&mut bytes)?);
            let (name, rest) = bytes.split_at(name_len as usize);
            bytes = rest;
            let name = String::from_utf8_lossy(name);
            names.insert(index, name);
        }
        return Ok(names);
    }

    return Ok(Default::default());
}

impl Symbols {
    /// Iterate over the symbols.
    pub fn iter(&self) -> SymbolsIter {
        // Find the set of function indices that are exported.
        let exports = self.module
            .export_section()
            .map_or(HashMap::new(), |section| {
                section
                    .entries()
                    .iter()
                    .filter_map(|entry| match *entry.internal() {
                        Internal::Function(idx) => Some((idx, entry.field())),
                        _ => None,
                    })
                    .collect()
            });

        let num_imports = self.module
            .import_section()
            .map_or(0, |imports| imports.entries().len());

        let names = self.module
            .sections()
            .iter()
            .filter_map(|section| match *section {
                Section::Custom(ref custom) if custom.name() == "name" => Some(custom),
                _ => None,
            })
            .next()
            .and_then(|name_section| decode_name_map(name_section.payload(), num_imports).ok());

        SymbolsIter {
            symbols: self,
            state: SymbolsIterState::new(self),
            exports,
            names,
        }
    }
}

/// An iterator returned by `Symbols::iter`, which iterates over the symbols in
/// a wasm file.
#[derive(Debug)]
pub struct SymbolsIter<'a> {
    symbols: &'a Symbols,
    state: SymbolsIterState<'a>,
    exports: HashMap<u32, &'a str>,
    names: Option<HashMap<u32, Cow<'a, str>>>,
}

#[derive(Debug)]
enum SymbolsIterState<'a> {
    Imports(slice::Iter<'a, ImportEntry>),
    Functions(iter::Enumerate<slice::Iter<'a, FuncBody>>),
    Finished,
}

impl<'a> SymbolsIterState<'a> {
    fn new(symbols: &'a Symbols) -> SymbolsIterState<'a> {
        SymbolsIterState::Imports(if let Some(section) = symbols.module.import_section() {
            section.entries().iter()
        } else {
            [].iter()
        })
    }
}

fn function_size(index: usize, module: &Module) -> Option<usize> {
    module
        .code_section()
        .and_then(|section| section.bodies().iter().nth(index))
        .and_then(|body| {
            let mut encoded = vec![];
            if let Err(_) = body.code().clone().serialize(&mut encoded) {
                return None;
            }
            Some(encoded.len())
        })
}

impl<'a> Iterator for SymbolsIter<'a> {
    type Item = Symbol<'a>;

    fn next(&mut self) -> Option<Symbol<'a>> {
        loop {
            self.state = match self.state {
                SymbolsIterState::Finished => return None,
                SymbolsIterState::Imports(ref mut imports) => match (
                    self.symbols.opts.imports,
                    imports.next(),
                ) {
                    (true, Some(import)) => {
                        return Some(Symbol::Import {
                            name: import.field(),
                        })
                    }
                    (false, _) | (true, None) => SymbolsIterState::Functions(
                        if let Some(section) = self.symbols.module.code_section() {
                            section.bodies().iter().enumerate()
                        } else {
                            [].iter().enumerate()
                        },
                    ),
                },
                SymbolsIterState::Functions(ref mut functions) => {
                    let (i, function) = match functions.next() {
                        Some(next) => next,
                        _ => break,
                    };
                    match (i, function, self.exports.get(&(i as u32))) {
                        (i, _, Some(export)) if self.symbols.opts.exports => {
                            return Some(Symbol::Export {
                                name: export,
                                size: if self.symbols.opts.sizes {
                                    function_size(i, &self.symbols.module)
                                } else {
                                    None
                                },
                            });
                        }
                        (i, _function, None) if self.symbols.opts.privates => {
                            let i = i as u32;
                            let name = self.names.as_ref().and_then(|names| names.get(&i).cloned());
                            return Some(Symbol::Private {
                                index: i,
                                name,
                                size: if self.symbols.opts.sizes {
                                    function_size(i as usize, &self.symbols.module)
                                } else {
                                    None
                                },
                            });
                        }
                        _ => {
                            continue;
                        }
                    }
                }
            };
        }

        self.state = SymbolsIterState::Finished;
        None
    }
}

/// A symbol from a wasm file.
#[derive(Clone, Debug)]
pub enum Symbol<'a> {
    /// An imported symbol.
    Import {
        /// The symbol's name.
        name: &'a str,
    },

    /// An exported symbol.
    Export {
        /// The symbol's name.
        name: &'a str,
        /// The symbol's size, if available.
        size: Option<usize>,
    },

    /// A private function that is not exported.
    Private {
        /// The function table index for this private function.
        index: u32,
        /// The name from the name section, if that information exists.
        name: Option<Cow<'a, str>>,
        /// The symbol's size, if available.
        size: Option<usize>,
    },
}

impl<'a> fmt::Display for Symbol<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Symbol::Import { name } | Symbol::Export { name, .. } => f.write_str(name),
            Symbol::Private {
                name: Some(ref name),
                ..
            } => f.write_str(&name),
            Symbol::Private { index, .. } => write!(f, "function[{}]", index),
        }
    }
}
