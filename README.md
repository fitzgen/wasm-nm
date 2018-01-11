# wasm-nm

[![](https://docs.rs/wasm-nm/badge.svg)](https://docs.rs/wasm-nm/) [![](https://img.shields.io/crates/v/wasm-nm.svg)](https://crates.io/crates/wasm-nm) [![](https://img.shields.io/crates/d/wasm-nm.png)](https://crates.io/crates/wasm-nm) [![Build Status](https://travis-ci.org/fitzgen/wasm-nm.png?branch=master)](https://travis-ci.org/fitzgen/wasm-nm)

List the symbols within a wasm file.

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

#### Using `wasm-nm` as a Size Profiler

`wasm-nm` can function as a rudimentary size profiler for `.wasm` files.

The `-z` option enables printing a function's code size. The unix `sort` utility
can be used to sort the symbols by size. The `rustfilt` utility can be used to
demangle Rust symbols (`cargo install rustfilt`).

```
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
