
extern crate ebpf as lib_ebpf;

use lib_ebpf::v1 as ebpf;
use ebpf::map::{
    Map,
    MutableMap
};

fn main() {

    println!("Creating eBPF map (type 'array') to hold 512 entries.");
    let mut array = ebpf::map::Array::new(64).expect("Array creation failed!");

    println!("Setting key 1 to value 101.");
    array.set(1,101).expect("Write failed!");
    println!("Getting value for key 1.");
    let v = array.get(1).expect("Read failed!");

    println!("Asserting value retrieved equals 101.");
    assert_eq!(v, 101);
}
