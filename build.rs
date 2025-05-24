use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    // Используем OUT_DIR или fallback
    let out_dir = match env::var("OUT_DIR") {
        Ok(val) => PathBuf::from(val),
        Err(_) => {
            let fallback = PathBuf::from("target/metaphone-build");
            fs::create_dir_all(&fallback).expect("Failed to create fallback build directory");
            fallback
        }
    };

    // Папка, где лежит Go-код и go.mod
    let go_dir = Path::new("lib");
    let output_lib = out_dir.join(shared_lib_name());

    // Команда go build
    let status = Command::new("go")
        .args([
            "build",
            "-buildmode=c-shared",
            "-o",
            output_lib.to_str().unwrap(),
            "metaphone3.go",
        ])
        .current_dir(go_dir)
        .status()
        .expect("Failed to execute Go build");

    if !status.success() {
        panic!("Go build failed with status: {status}");
    }

    // Линкуем
    println!("cargo:rustc-link-lib=dylib=metaphone3");
    println!("cargo:rustc-link-search=native={}", out_dir.display());

    // Пересборка при изменении
    println!("cargo:rerun-if-changed=lib/metaphone3.go");
    println!("cargo:rerun-if-changed=lib/go.mod");
}

fn shared_lib_name() -> &'static str {
    #[cfg(target_os = "macos")]
    { "libmetaphone3.dylib" }

    #[cfg(target_os = "linux")]
    { "libmetaphone3.so" }

    #[cfg(target_os = "windows")]
    { "metaphone3.dll" }

}
