pub fn main() {
    // This is a trick to enable the f16 and f128 features whenever 
    // this crate is built on nightly
    if let Ok(toolchain) = std::env::var("RUSTUP_TOOLCHAIN") {
        if toolchain.starts_with("nightly") {
            println!("cargo:rustc-cfg=nightly");
        }
    }
    println!("cargo:rustc-check-cfg=cfg(nightly)");
}