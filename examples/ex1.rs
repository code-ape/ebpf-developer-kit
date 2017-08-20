
extern crate ebpf;

use std::io;
use ebpf::v1::map::{
    create_map, map_update_elem, map_lookup_elem
};

use ebpf::v1::data::{
    Action, Attr, MapCreateAttr, MapElemAttr
};

fn main() {
    let map_create_attr = MapCreateAttr {
        map_type: 1,
        key_size: 8,
        value_size: 8,
        max_entries: 512,
        map_flags: 0
    };

    println!("attr = {:?}", map_create_attr);
    println!("action (binary) = {:b}", Action::MapCreate);
    //let attr = Attr { map_create: map_create_attr };
    
    //println!("attr = \\\n{:b}", map_create_attr);

    let map = create_map(map_create_attr);

    println!("map = {:?}", map);

    let k : u64 = 1;
    let v : u64 = 33;

    let k_p : *const u64 = &k;
    let v_p : *const u64 = &v;

    let map_elem_attr = MapElemAttr {
        map_fd: map.clone().into(),
        key: k_p as u64,
        value_or_next_key: v_p as u64,
        flags: 1
    };

    let w = map_update_elem(map_elem_attr);
    println!("w = {:?}", w);


    let r_v : u64 = 0;
    let r_p : *const u64 = &r_v;
    let map_elem_attr_2 = MapElemAttr {
        map_fd: map.clone().into(),
        key: k_p as u64,
        value_or_next_key: r_p as u64,
        flags: 0
    };

    println!("r_v (pre) = {:?}", r_v);
    let r = map_lookup_elem(map_elem_attr_2);
    println!("r = {:?}", r);
    println!("r_v (post) = {:?}", r_v);

    //let mut s = String::new();
    //println!("Waiting for text... ");
    //io::stdin().read_line(&mut s).expect("Error reading input!");

}