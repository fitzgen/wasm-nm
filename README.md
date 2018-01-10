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
3578 p std::panicking::begin_panic_fmt::h47786a9a66db0de4
2078 p core::slice::slice_index_order_fail::h2c23bc1ce370b6f1
1324 p core::ptr::drop_in_place::hcd2d108484489df3
1268 p dlmalloc::dlmalloc::Dlmalloc::memalign::hee616eb0f35bbba8
1253 p std::io::Write::write_all::h1e22c345ee74bd20
1248 p core::fmt::num::<impl core::fmt::Debug for usize>::fmt::he64994cf6f0229ef
1064 p dlmalloc::dlmalloc::Dlmalloc::insert_large_chunk::h95b574ef6905303c
987 p dlmalloc::dlmalloc::Dlmalloc::dispose_chunk::hfb236c21060aea2f
978 e allocate_mappings
974 p source_map_mappings_wasm_api::LAST_ERROR::__getit::h52f017cac8e76e23
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
