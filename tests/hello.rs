#![cfg(target_arch = "wasm32")]

#[no_mangle]
pub fn fluxions(x: usize) -> usize {
    unsafe { imported(x) }
}

#[no_mangle]
pub fn quicksilver(_: usize) {}

extern {
    fn imported(x: usize) -> usize;
}

fn main() {}
