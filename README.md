# eBPF Rust (ebpf-rs)
Library for using eBPF from Rust

## References

### From Linux

1. Linux `bpf` syscall index is 321: [https://github.com/torvalds/linux/blob/master/arch/x86/entry/syscalls/syscall_64.tbl](https://github.com/torvalds/linux/blob/master/arch/x86/entry/syscalls/syscall_64.tbl)

### Other work for using eBPF with Rust

1. Frank Denis appears to have done very minor work for using Rust to load an eBPF program - [https://github.com/jedisct1/rust-bpf](https://github.com/jedisct1/rust-bpf)
2. Quentin Monnet has create an eBPF virtual machine and JIT compiler in Rust called rbpf - [https://github.com/qmonnet/rbpf](https://github.com/qmonnet/rbpf)
3. Libpnet has some usage of eBPF but seems very minor - [https://github.com/libpnet/libpnet](https://github.com/libpnet/libpnet)
