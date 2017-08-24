
extern crate ebpf_toolchain;

use std::panic;

use ebpf_toolchain::v1 as ebpf;

pub use ebpf::map::{
    Array,
    Map,
    MutableMap
};

#[no_mangle]
pub extern "C" fn try_this() -> i32 {
    let r = panic::catch_unwind(|| {
        let mut a = Array::new(1024).unwrap();
        a.set(1,101i32).unwrap();
        return a.get(1).unwrap();
    });
    match r {
        Ok(n) => n,
        Err(_) => -1
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
