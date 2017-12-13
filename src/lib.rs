/*!

[![](https://docs.rs/wasm-nm/badge.svg)](https://docs.rs/wasm-nm/) [![](https://img.shields.io/crates/v/wasm-nm.svg)](https://crates.io/crates/wasm-nm) [![](https://img.shields.io/crates/d/wasm-nm.png)](https://crates.io/crates/wasm-nm) [![Build Status](https://travis-ci.org/fitzgen/wasm-nm.png?branch=master)](https://travis-ci.org/fitzgen/wasm-nm)

List the imported and exported symbols within a wasm file.

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

#[macro_use]
extern crate failure;
extern crate parity_wasm;

use parity_wasm::elements::{Deserialize, ExportEntry, ImportEntry, Module};
use std::fmt;
use std::io;
use std::slice;

// Needed until https://github.com/paritytech/parity-wasm/issues/125 is fixed.
#[derive(Debug, Fail)]
#[fail(display = "{:?}", _0)]
struct WasmError(parity_wasm::elements::Error);

/// Options for controlling which symbols are iterated over.
#[derive(Clone, Debug)]
pub struct Options {
    /// Should imported symbols be iterated over?
    pub imports: bool,
    /// Should exported symbols be iterated over?
    pub exports: bool,
}

impl Default for Options {
    fn default() -> Options {
        Options {
            imports: true,
            exports: true,
        }
    }
}

/// Get the symbols in the given wasm file.
pub fn symbols<R>(opts: Options, reader: &mut R) -> Result<Symbols, failure::Error>
where
    R: io::Read,
{
    let module = Module::deserialize(reader).map_err(WasmError)?;
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

impl Symbols {
    /// Iterate over the symbols.
    pub fn iter(&self) -> SymbolsIter {
        SymbolsIter {
            symbols: self,
            state: SymbolsIterState::new(self),
        }
    }
}

/// An iterator returned by `Symbols::iter`, which iterates over the symbols in
/// a wasm file.
#[derive(Debug)]
pub struct SymbolsIter<'a> {
    symbols: &'a Symbols,
    state: SymbolsIterState<'a>,
}

#[derive(Debug)]
enum SymbolsIterState<'a> {
    Imports(slice::Iter<'a, ImportEntry>),
    Exports(slice::Iter<'a, ExportEntry>),
    Finished,
}

impl<'a> SymbolsIterState<'a> {
    fn new(symbols: &'a Symbols) -> SymbolsIterState<'a> {
        if symbols.opts.imports {
            if let Some(section) = symbols.module.import_section() {
                return SymbolsIterState::Imports(section.entries().iter());
            }
        }

        if symbols.opts.exports {
            if let Some(section) = symbols.module.export_section() {
                return SymbolsIterState::Exports(section.entries().iter());
            }
        }

        SymbolsIterState::Finished
    }
}

impl<'a> Iterator for SymbolsIter<'a> {
    type Item = Symbol<'a>;

    fn next(&mut self) -> Option<Symbol<'a>> {
        loop {
            self.state = match self.state {
                SymbolsIterState::Imports(ref mut imports) => match imports.next() {
                    Some(import) => return Some(Symbol::Import(import.field())),
                    None => if self.symbols.opts.exports {
                        if let Some(section) = self.symbols.module.export_section() {
                            SymbolsIterState::Exports(section.entries().iter())
                        } else {
                            SymbolsIterState::Finished
                        }
                    } else {
                        SymbolsIterState::Finished
                    }
                },
                SymbolsIterState::Exports(ref mut exports) => match exports.next() {
                    Some(export) => return Some(Symbol::Export(export.field())),
                    None => SymbolsIterState::Finished,
                },
                SymbolsIterState::Finished => return None,
            };
        }
    }
}

/// A symbol from a wasm file.
#[derive(Clone, Debug)]
pub enum Symbol<'a> {
    /// An imported symbol.
    Import(&'a str),

    /// An exported symbol.
    Export(&'a str),
}

impl<'a> fmt::Display for Symbol<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Symbol::Import(s) | Symbol::Export(s) => f.write_str(s),
        }
    }
}
