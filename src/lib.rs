//! TODO FITZGEN

#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

#[macro_use]
extern crate failure;
extern crate parity_wasm;

use parity_wasm::elements::{Deserialize, ExportEntry, ImportEntry, Module};
use std::fmt;
use std::io;
use std::slice;

/*
Each symbol name is preceded by its value (blanks if undefined).  Unless the -m
option is specified, this value is followed by one of the following characters,
representing the symbol type: U (unde- fined), A (absolute), T (text section
symbol), D (data section symbol), B (bss section symbol), C (common symbol), -
(for debugger symbol table entries; see -a below), S (symbol in a section other
than those above), or I (indirect symbol).  If the symbol is local
(non-external), the symbol's type is instead represented by the corresponding
lowercase letter.  A lower case u in a dynamic shared library indicates a
undefined reference to a private external in another module in the same library.
*/

/// TODO FITZGEN
#[derive(Debug, Fail)]
#[fail(display = "{:?}", _0)]
pub struct WasmError(parity_wasm::elements::Error);

/// TODO FITZGEN
#[derive(Clone, Debug)]
pub struct Options {
    /// TODO FITZGEN
    pub imports: bool,
    /// TODO FITZGEN
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

/// TODO FITZGEN
pub fn symbols<R>(opts: Options, reader: &mut R) -> Result<Symbols, failure::Error>
where
    R: io::Read,
{
    let module = Module::deserialize(reader).map_err(WasmError)?;
    Ok(Symbols { opts, module })
}

/// TODO FITZGEN
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
    /// TODO FITZGEN
    pub fn iter(&self) -> SymbolsIter {
        SymbolsIter {
            symbols: self,
            state: SymbolsIterState::new(self),
        }
    }
}

/// TODO FITZGEN
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

/// TODO FITZGEN
#[derive(Clone, Debug)]
pub enum Symbol<'a> {
    /// TODO FITZGEN
    Import(&'a str),

    /// TODO FITZGEN
    Export(&'a str),
}

impl<'a> fmt::Display for Symbol<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Symbol::Import(s) | Symbol::Export(s) => f.write_str(s),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
