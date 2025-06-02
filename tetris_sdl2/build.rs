use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let debug_dir = Path::new(&out_dir)
        .ancestors()
        .nth(3) // OUT_DIR/…/…/… ≈ target/debug
        .expect("Unable to determine target dir");

    let dlls = [
        "SDL2.dll",
        "SDL2_image.dll",
        "SDL2_ttf.dll",
    ];

    for dll in &dlls {
        let src = Path::new("lib/x64").join(dll);
        let dst = debug_dir.join(dll);
        match fs::copy(&src, &dst) {
            Ok(_) => println!("cargo:warning=Copied {} to {:?}", dll, dst),
            Err(e) => println!("cargo:warning=Failed to copy {}: {}", dll, e),
        }
    }

    cc::Build::new()
        .file("test.c")
        .compile("suker_test");

    #[cfg(target_os = "windows")]
    println!("cargo:rustc-cfg=SUKER_WIN");
    #[cfg(target_os = "linux")]
    println!("cargo:rustc-cfg=SUKER_LINUX");
}