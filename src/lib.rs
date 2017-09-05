//! Library for using eBPF from Rust (currently pre-alpha)

#![cfg(target_os = "linux")]

extern crate libc;

#[cfg(feature = "elf_loader")]
extern crate xmas_elf;

#[cfg(feature = "socket_filter")]
extern crate networking as lib_networking;

pub mod v1;