#![feature(abi_efiapi)]

#[cfg(not(target_arch = "x86_64"))]
compile_error!("This crate only builds on x86_64 machines");

use core::arch::global_asm;

// Cargo rebuilds this automatically, if the file changes.
global_asm!(include_str!("global_asm.S"));

// see https://github.com/rust-lang/rust/blob/b09dad3eddfc46c55e45f6c1a00bab09401684b4/compiler/rustc_target/src/spec/abi.rs
// for possible values here


// Behind the scenes, Windows uses "Microsoft/PE" calling convention.
//   https://docs.microsoft.com/en-us/cpp/build/x64-calling-convention?view=msvc-160
extern "win64" {
    fn win64_abi__asm_add(a: i64, b: i64) -> i64;
}

// Behind the scenes, UEFI uses "Microsoft/PE" calling convention.
//   https://docs.microsoft.com/en-us/cpp/build/x64-calling-convention?view=msvc-160
extern "efiapi" {
    fn efi_abi__asm_add(a: i64, b: i64) -> i64;
}

// Defaults to System V ABI (64 bit), i.e. the calling convention used on
// Linux or MacOS (x86_64).
//   https://www.uclibc.org/docs/psABI-x86_64.pdf
extern "sysv64" {
    fn system_v_abi__asm_add(a: i64, b: i64) -> i64;
}

/// Calculates two numbers by using functions of two different calling conventions.
/// Furthermore, this example shows that "win64" and "efiapi" use indeed the "Microsoft/PE"
/// calling convention.
fn main() {
    println!("win64_abi__asm_add(2,7)   = {}", unsafe { win64_abi__asm_add(2, 7) });
    println!("efi_abi__asm_add(2,7)     = {}", unsafe { efi_abi__asm_add(2, 7) });
    println!("system_v_abi__asm_add(2,7)= {}", unsafe { system_v_abi__asm_add(2, 7) });
}
