use std::process::Command;

const OVMF_PATH: &'static str = "/usr/share/edk2/ovmf/";

fn main() {
    let status = Command::new("cargo")
        .arg("build")
        .arg("--package")
        .arg("blasterball")
        .arg("-Z")
        .arg("build-std=core")
        .arg("--target")
        .arg("x86_64-unknown-uefi")
        .status()
        .unwrap();
    if !status.success() {
        eprintln!("Failed to build the game");
        std::process::exit(1);
    }

    let status = Command::new("sudo")
        .arg("qemu-system-x86_64")
        .arg("-enable-kvm")
        .arg("-vga")
        .arg("std")
        .arg("-display")
        .arg("gtk")
        .arg("-nodefaults")
        .arg("-drive")
        .arg(&format!("if=pflash,format=raw,unit=0,file={}/OVMF_CODE.fd,readonly=on", OVMF_PATH))
        .arg("-drive")
        .arg(&format!("if=pflash,unit=1,format=raw,file={}/OVMF_VARS.fd", OVMF_PATH))
        .arg("-drive")
        .arg("format=raw,file=fat:rw:target/x86_64-unknown-uefi/debug/")
        .status()
        .unwrap();
    if !status.success() {
        eprintln!("Failed to start emulator");
        std::process::exit(1);
    }
}
