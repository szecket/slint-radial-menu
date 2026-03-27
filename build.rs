// Copyright © 2026
// SPDX-License-Identifier: MIT

fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:slint_library_path={}/ui", manifest_dir);
    
    slint_build::compile("ui/radial-menu.slint").unwrap();
}
