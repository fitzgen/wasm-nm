To regenerate the wasm files:

```
$ rustc +nightly --target wasm32-unknown-unknown -O -g ./hello.rs
$ wasm-gc hello.wasm small-hello.wasm
```
