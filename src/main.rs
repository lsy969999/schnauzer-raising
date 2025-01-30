#[cfg(not(any(target_os = "ios", target_os = "android", target_family = "wasm")))]
fn main() {
    schnauzer_raising::main();
}

#[cfg(any(target_os = "ios", target_os = "android", target_family = "wasm"))]
fn main() {}
