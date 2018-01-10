#![cfg(target_arch = "wasm32")]

#[no_mangle]
pub fn fluxions(x: usize) -> usize {
    unsafe { imported(x) }
}

#[no_mangle]
pub fn quicksilver(_: usize) {}

extern "C" {
    fn imported(x: usize) -> usize;
}

#[no_mangle]
pub fn main() {}
