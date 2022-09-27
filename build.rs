use std::{env, path::Path};

fn main() {
    let dir = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let dir = Path::new(&dir);

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rerun-if-changed=stack.x");
    println!("cargo:rustc-link-search={}", dir.display());
    println!("cargo:rustc-link-arg=-Tcortex-a-rt.x");
    println!("cargo:rustc-link-arg=-Tdefmt.x");
}
