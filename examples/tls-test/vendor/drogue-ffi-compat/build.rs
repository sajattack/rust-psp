extern crate cc;

use std::{
    env,
    path::PathBuf,
};

fn main() {
    let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let include_dir = PathBuf::from(&project_dir).join("include");
    println!("cargo:include={}", include_dir.display());

    let mut b = cc::Build::new();
    
    b.file("src/printf.c");
    b.include(include_dir);
    b.static_flag(true);
    b.compile("libdrogue-ffi-compat-shim.a");
}
