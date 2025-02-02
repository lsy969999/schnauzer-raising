mod app;
mod asset;
mod crossbeam;
mod ffi;
#[cfg(feature = "inspector")]
mod inspector;
mod tokio;
mod ui;

#[bevy::prelude::bevy_main]
#[cfg_attr(target_family = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub fn main() {
    use ffi::FfiAppInterface;
    ffi::FfiApp::main();
}
