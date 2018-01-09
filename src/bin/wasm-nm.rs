extern crate clap;
extern crate failure;
extern crate wasm_nm;

use std::fs;
use std::io;
use std::process;

fn main() {
    if let Err(()) = try_main() {
        process::exit(1)
    }
}

fn try_main() -> Result<(), ()> {
    let matches = parse_args();

    let mut opts = wasm_nm::Options::default();

    if matches.is_present("only_imports") {
        opts = wasm_nm::Options::nothing();
        opts.imports = true;
    } else if matches.is_present("only_exports") {
        opts = wasm_nm::Options::nothing();
        opts.exports = true;
    } else if matches.is_present("only_privates") {
        opts = wasm_nm::Options::nothing();
        opts.privates = true;
    }

    let mut any_errors = false;
    for path in matches.values_of("file").unwrap() {
        if let Err(e) = print_symbols_in_one(path, opts.clone(), &matches) {
            eprintln!("error: {}: {}", path, e);
            any_errors = true;
        }
    }

    if any_errors {
        Err(())
    } else {
        Ok(())
    }
}

fn print_symbols_in_one(
    path: &str,
    opts: wasm_nm::Options,
    matches: &clap::ArgMatches<'static>,
) -> Result<(), failure::Error> {
    let file = fs::File::open(path)?;
    let mut file = io::BufReader::new(file);

    let symbols = wasm_nm::symbols(opts, &mut file)?;

    let just_symbols = matches.is_present("just_symbols");
    for sym in symbols.iter() {
        if !just_symbols {
            match sym {
                wasm_nm::Symbol::Import(_) => print!("i "),
                wasm_nm::Symbol::Export(_) => print!("e "),
                wasm_nm::Symbol::Private(..) => print!("p "),
            }
        }
        println!("{}", sym);
    }

    Ok(())
}

fn parse_args() -> clap::ArgMatches<'static> {
    clap::App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .long_about(
            "\
wasm-nm displays imported and exported symbols in a wasm file.

Each symbol is preceded by its symbol type: \"i\" for imported symbols, \"e\"
for exported symbols, and \"p\" for private symbols. Alternatively, the -j \
flag can be used to avoid any prefixes.\
",
        )
        .arg(
            clap::Arg::with_name("file")
                .required(true)
                .multiple(true)
                .help("The wasm file(s) whose symbols should be dumped."),
        )
        .arg(
            clap::Arg::with_name("just_symbols")
                .short("j")
                .help("Just display the symbol names (no type)."),
        )
        .arg(
            clap::Arg::with_name("only_imports")
                .short("i")
                .conflicts_with("only_exports")
                .conflicts_with("only_privates")
                .help("Display only import symbols."),
        )
        .arg(
            clap::Arg::with_name("no_imports")
                .short("I")
                .conflicts_with("only_imports")
                .help("Do not display import symbols."),
        )
        .arg(
            clap::Arg::with_name("only_exports")
                .short("e")
                .conflicts_with("only_imports")
                .conflicts_with("only_privates")
                .help("Display only export symbols."),
        )
        .arg(
            clap::Arg::with_name("no_exports")
                .short("E")
                .conflicts_with("only_exports")
                .help("Do not display export symbols."),
        )
        .arg(
            clap::Arg::with_name("only_privates")
                .short("p")
                .conflicts_with("only_exports")
                .conflicts_with("only_imports")
                .help("Display only private symbols."),
        )
        .arg(
            clap::Arg::with_name("no_privates")
                .short("P")
                .conflicts_with("only_privates")
                .help("Do not display private symbols."),
        )
        .get_matches()
}
