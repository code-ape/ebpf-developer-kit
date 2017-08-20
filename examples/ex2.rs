
extern crate ebpf as lib_ebpf;

use lib_ebpf::v1 as ebpf;
use ebpf::map::{
    WriteOption,
    WritableMap,
    Map
};

fn main() {

    let mut hm = ebpf::map::HashMap::<usize,usize>::new(512)
                .expect("Hashmap creation failed!");

    hm.set(1,101, WriteOption::CreateEntry).expect("Write failed!");
    let v = hm.get(1).expect("Read failed!");

    assert_eq!(v, 101);
}