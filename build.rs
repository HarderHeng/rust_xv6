use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=startup.S");
    println!("cargo:rerun-if-changed=link.x");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // 1. 汇编 startup.S -> startup.o
    let asm_obj = out_dir.join("startup.o");
    let status = Command::new("riscv64-unknown-elf-as")
        .args(&[
            "-march=rv32im",
            "-mabi=ilp32",
            "-o",
            asm_obj.to_str().unwrap(),
            "startup.S",
        ])
        .status()
        .expect("Failed to assemble startup.S");

    if !status.success() {
        panic!("Assembling startup.S failed");
    }

    // 2. 将 startup.o 打包为 libstartup.a
    let archive = out_dir.join("libstartup.a");
    let status = Command::new("ar")
        .args(&["crs", archive.to_str().unwrap(), asm_obj.to_str().unwrap()])
        .status()
        .expect("Failed to run 'ar'");

    if !status.success() {
        panic!("Archiving libstartup.a failed");
    }

    // 3. 告诉 Rust 链接器在哪里找 libstartup.a
    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=startup");
}