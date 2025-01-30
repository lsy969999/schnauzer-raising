#[cfg(not(any(target_os = "ios", target_os = "android", target_family = "wasm")))]
pub mod desktop;

#[cfg(target_family = "wasm")]
pub mod wasm;

#[cfg(target_os = "ios")]
pub mod ios;

pub struct FfiApp;

pub trait FfiAppInterface {
    fn main();
}
