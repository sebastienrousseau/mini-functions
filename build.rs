use std::env;
use std::process::Command;
use std::str;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    // Avoid unnecessary re-building.
    println!("cargo:rerun-if-changed=src/date.rs");
    println!("cargo:rerun-if-changed=src/log.rs");
    println!("cargo:rerun-if-changed=src/qrcode.rs");
    println!("cargo:rerun-if-changed=src/uuid.rs");

    // Generate the documentation.
    if let Some(compiler) = rustc_version() {
        if compiler.minor >= 51 && !compiler.nightly {
            println!("cargo:rustc-cfg=doc_cfg");
        }
    }
    if let Some(compiler) = rustc_version() {
        if compiler.minor >= 51 && !compiler.nightly {
            println!("cargo:rustc-cfg=doc_cfg");
        }
    }
}

struct Compiler {
    minor: u32,
    nightly: bool,
}

fn rustc_version() -> Option<Compiler> {
    let rustc = env::var_os("RUSTC")?;
    let output = Command::new(rustc).arg("--version").output().ok()?;
    let version = str::from_utf8(&output.stdout).ok()?;
    let mut pieces = version.split('.');
    if pieces.next() != Some("rustc 1") {
        return None;
    }
    let minor = pieces.next()?.parse().ok()?;
    let nightly = version.contains("nightly") || version.ends_with("-dev");
    Some(Compiler { minor, nightly })
}
