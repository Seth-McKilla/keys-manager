#![cfg_attr(
    not(target_arch = "wasm32"),
    crate_type = "target arch should be wasm32"
)]
#![no_main]

#[no_mangle]
pub extern "C" fn call() {
    keys_manager::execute();
}
