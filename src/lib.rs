//! Library for using eBPF from Rust (currently pre-alpha)

#![cfg(target_os = "linux")]

extern crate libc;

#[cfg(feature = "elf_loader")]
extern crate xmas_elf;

pub mod v1;