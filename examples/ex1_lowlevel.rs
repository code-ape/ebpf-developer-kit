
extern crate ebpf_development_kit;

use ebpf_development_kit::v1 as ebpf;

use ebpf::map::lowlevel::{
    create_map, map_update_elem, map_lookup_elem
};

use ebpf::lowlevel::{
    Action, MapCreateAttr, MapElemAttr, MapType
};

fn main() {
    let map_create_attr = MapCreateAttr {
        map_type: MapType::Hash as u32,
        key_size: 8,
        value_size: 8,
        max_entries: 512,
        map_flags: 0
    };

    println!("attr = {:?}", map_create_attr);
    println!("attr (binary) = \n{:b}", map_create_attr);
    println!("action (binary) = \n{:b}", Action::MapCreate);

    let map = unsafe { create_map(map_create_attr).unwrap() };

    println!("map = {:?}", map);

    let k : u64 = 1;
    let v : u64 = 101;

    let k_p : *const u64 = &k;
    let v_p : *const u64 = &v;

    let map_elem_attr = MapElemAttr {
        map_fd: unsafe { map.get_fd() } as u32,
        key: k_p as u64,
        value_or_next_key: v_p as u64,
        flags: 1
    };

    let w = unsafe { map_update_elem(map_elem_attr) };
    println!("w = {:?}", w);


    let r_v : u64 = 0;
    let r_p : *const u64 = &r_v;
    let map_elem_attr_2 = MapElemAttr {
        map_fd: unsafe { map.get_fd() } as u32,
        key: k_p as u64,
        value_or_next_key: r_p as u64,
        flags: 0
    };

    println!("r_v (pre) = {:?}", r_v);
    let r = unsafe { map_lookup_elem(map_elem_attr_2) };
    println!("r = {:?}", r);
    println!("r_v (post) = {:?}", r_v);

    assert_eq!(r_v, 101);

}
