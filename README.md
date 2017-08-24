# eBPF Rust (ebpf-rs)
Library for using eBPF from Rust

## Trying an example

Let's look at `examples/ex1.rs` which contains the following code:

```rust
extern crate ebpf as lib_ebpf;

use lib_ebpf::v1 as ebpf;
use ebpf::map::{
    Map,
    WritableMap,
    WriteOption
};

fn main() {

    println!("Creating eBPF map (type 'hashmap') to hold 512 entries.");
    let mut hm = ebpf::map::HashMap::new(512).expect("Hashmap creation failed!");

    println!("Setting key 1 to value 101.");
    hm.set(1,101, WriteOption::CreateEntry).expect("Write failed!");
    println!("Getting value for key 1.");
    let v = hm.get(1).expect("Read failed!");

    println!("Asserting value retrieved equals 101.");
    assert_eq!(v, 101);
}
```

We can run this code by doing the following:

```
$ cargo run --example ex1
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/examples/ex1`
Creating eBPF map (type 'hashmap') to hold 512 entries.
Setting key 1 to value 101.
Getting value for key 1.
Asserting value retrieved equals 101.
```

You can also run the binary with strace tracking bpf syscalls, which will print the all the bpf syscalls and their arguments.
This is useful to validate that the functions do in fact do their expected syscalls.

```
$ cargo build --example ex1 && sudo strace -e trace=bpf target/debug/examples/ex1
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
Creating eBPF map (type 'hashmap') to hold 512 entries.
bpf(BPF_MAP_CREATE, {map_type=BPF_MAP_TYPE_HASH, key_size=4, value_size=4, max_entries=512}, 20) = 3
Setting key 1 to value 101.
bpf(BPF_MAP_UPDATE_ELEM, {map_fd=3, key=0x7ffc6a8e9754, value=0x7ffc6a8e9758, flags=BPF_NOEXIST}, 32) = 0
Getting value for key 1.
bpf(BPF_MAP_LOOKUP_ELEM, {map_fd=3, key=0x7ffc6a8e9720, value=0x7ffc6a8e9724}, 32) = 0
Asserting value retrieved equals 101.
+++ exited with 0 +++
```

## Understanding the libraries structure

This library attempts to make it as simple as possible to factor in for all the following things:

1. Version of the Linux Kernel you're using. Because different eBPF features were added during different versions.
2. The stability of the library. This is so you can use the library in stable, beta, or alpha; which means you can use the library with confidence (assuming you stick to stable) that the API won't change.
3. The versioning of the library. Because this librarys operates at such a low level, changes API's for it can dramatically hurt users. Thus each version of the library lives in it's own namespace. This means that if you build on version 1 and later version two is done with a better API, then you can continue to use version one in legacy systems while also using version 2 without worry. This means that anything in ebpf-rs marked stable is **guarenteed** to never move or change as you use newer versions.

## References

### From Linux

1. Linux `bpf` syscall index is 321: [https://github.com/torvalds/linux/blob/master/arch/x86/entry/syscalls/syscall_64.tbl](https://github.com/torvalds/linux/blob/master/arch/x86/entry/syscalls/syscall_64.tbl)

### Other work for using eBPF with Rust

1. Frank Denis appears to have done very minor work for using Rust to load an eBPF program - [https://github.com/jedisct1/rust-bpf](https://github.com/jedisct1/rust-bpf)
2. Quentin Monnet has create an eBPF virtual machine and JIT compiler in Rust called rbpf - [https://github.com/qmonnet/rbpf](https://github.com/qmonnet/rbpf)
3. Libpnet has some usage of eBPF but seems very minor - [https://github.com/libpnet/libpnet](https://github.com/libpnet/libpnet)
