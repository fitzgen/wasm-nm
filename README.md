# wasm-nm

[![](https://docs.rs/wasm-nm/badge.svg)](https://docs.rs/wasm-nm/) [![](https://img.shields.io/crates/v/wasm-nm.svg)](https://crates.io/crates/wasm-nm) [![](https://img.shields.io/crates/d/wasm-nm.png)](https://crates.io/crates/wasm-nm) [![Build Status](https://travis-ci.org/fitzgen/wasm-nm.png?branch=master)](https://travis-ci.org/fitzgen/wasm-nm)

List the imported and exported symbols within a wasm file.

* [Library](#library)
* [Executable](#executable)
* [License](#license)
* [Contributing](#contributing)

### Executable

To install the `wasm-nm` executable, run

```
$ cargo install wasm-nm
```

For information on using the `wasm-nm` executable, run

```
$ wasm-nm --help
```

### Library

To use `wasm-nm` as a library, add this to your `Cargo.toml`:

```toml
[dependencies.wasm-nm]
# Do not build the executable.
default-features = false
```

See [docs.rs/wasm-nm][docs] for API documentation.

[docs]: https://docs.rs/wasm-nm

### License

Licensed under either of

 * [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)

 * [MIT license](http://opensource.org/licenses/MIT)

at your option.

### Contributing

See
[CONTRIBUTING.md](https://github.com/fitzgen/wasm-nm/blob/master/CONTRIBUTING.md)
for hacking.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.


License: Apache-2.0/MIT
