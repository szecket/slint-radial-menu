// Copyright © 2026
// SPDX-License-Identifier: MIT

fn main() {
    // Expose the UI directory path to dependent crates
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let ui_path = std::path::Path::new(&manifest_dir).join("ui");
    println!("cargo:SLINT_LIBRARY_PATH={}", ui_path.display());
    
    slint_build::compile("ui/radial-menu.slint").unwrap();
}
