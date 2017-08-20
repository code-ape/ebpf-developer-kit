
//! Version 1 of eBPF library.

pub mod map;
pub mod data;

use std::mem;

use libc::{
    c_int,
    c_long,
    SYS_bpf,
    syscall as linux_syscall,
};

use self::data::{
    Attr,
    Action,
};

// __aligned_u64
// defined in https://github.com/torvalds/linux/blob/8b4822de59d5d9919b9b045183a36c673ce20b73/tools/testing/selftests/bpf/include/uapi/linux/types.h
// #define __aligned_u64 __u64 __attribute__((aligned(8)))


pub fn syscall<T: Attr>(action: Action, attr: T) -> c_long {
    let attr_size = mem::size_of::<T>() as c_int;
    let attr_raw_pointer: *const T = &attr;
    return unsafe {
        linux_syscall(
            SYS_bpf, action as c_int,
            attr_raw_pointer, mem::size_of::<T>() as c_int
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
