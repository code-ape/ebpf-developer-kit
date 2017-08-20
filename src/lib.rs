//! Library for using eBPF from Rust (currently pre-alpha)

#![feature(untagged_unions)]
#![feature(const_fn)]

extern crate libc;

pub mod v1;