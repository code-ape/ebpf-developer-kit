
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

    println!("Deleting value for key 1.");
    hm.delete(1).expect("Delete failed!");

    println!("Verifying no value for key 1.");
    assert!(hm.get(1).is_err());
}
