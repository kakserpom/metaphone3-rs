use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let profile = env::var("PROFILE").expect("PROFILE not set"); // debug or release
    let target = env::var("TARGET").expect("TARGET not set");
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));
    let target_dir = find_target_dir(&out_dir);

    let go_dir = Path::new("lib");
    let output_lib_path = out_dir.join(shared_lib_filename());

    // Build Go .so/.dll/.dylib
    let status = Command::new("go")
        .args([
            "build",
            "-buildmode=c-shared",
            "-o",
            output_lib_path.to_str().unwrap(),
            "metaphone3.go",
        ])
        .current_dir(go_dir)
        .status()
        .expect("Failed to execute `go build`");

    if !status.success() {
        panic!("Go build failed with status {status}");
    }

    // Copy to target/{triple}/{debug|release}
    let final_path = target_dir
        .join(&target)
        .join(&profile)
        .join(shared_lib_filename());
    fs::create_dir_all(final_path.parent().unwrap()).expect("Failed to create output dir");
    fs::copy(&output_lib_path, &final_path)
        .unwrap_or_else(|e| panic!("Failed to copy library to final location: {e}"));

    // Trigger rebuild if source files change
    println!("cargo:rerun-if-changed=lib/metaphone3.go");
    println!("cargo:rerun-if-changed=lib/go.mod");

    if env::var("CARGO_CFG_TARGET_OS").unwrap().as_str() != "windows" {
        println!("cargo:rustc-link-lib=dylib=metaphone3");
        println!("cargo:rustc-link-search=native={}", out_dir.display());
    }
}

fn shared_lib_filename() -> &'static str {
    match env::var("CARGO_CFG_TARGET_OS").unwrap().as_str() {
        "windows" => "metaphone3.dll",
        "macos" => "libmetaphone3.dylib",
        "linux" => "libmetaphone3.so",
        other => panic!("Unsupported target OS: {other}"),
    }
}

// Walks up to find the "target/" directory
fn find_target_dir(mut path: &Path) -> PathBuf {
    while let Some(parent) = path.parent() {
        if parent.ends_with("target") {
            return parent.to_path_buf();
        }
        path = parent;
    }
    panic!("Could not find target/ directory");
}
