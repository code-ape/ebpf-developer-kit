
pub mod data;

use libc::{syscall, c_int, c_long, __u32, __u64, SYS_bpf};
use std::os::unix::io::RawFd;
use std::mem;
use std::fmt;
use std::io::Error;

use ::v1::data::{
    MapCreateAttr,
    MapElemAttr,
    MapId,
};

pub use ::v1::data::Action;

pub use self::data::{
    WriteOption,
    Map,
    WritableMap,
    HashMap
};


use ::v1::syscall as ebpf_syscall;

pub fn create_map(map_create_attr: MapCreateAttr) -> Result<MapId,Error> {
    match ebpf_syscall(Action::MapCreate, map_create_attr) {
        n if n > 0 => Ok(MapId::new(n)),
        -1 => Err(Error::last_os_error()),
        n => unreachable!("Syscall returned number other than 0 or 1: {}", n)
    }
}


pub fn map_update_elem(map_elem_attr: MapElemAttr) -> Result<(),Error> {
    match ebpf_syscall(Action::MapUpdateElem, map_elem_attr) {
        0 => Ok(()),
        -1 => Err(Error::last_os_error()),
        n => unreachable!("Syscall returned number other than 0 or 1: {}", n)
    }
}


pub fn map_lookup_elem(map_elem_attr: MapElemAttr) -> Result<(),Error> {
    match ebpf_syscall(Action::MapLookupElem, map_elem_attr) {
        0 => Ok(()),
        -1 => Err(Error::last_os_error()),
        n => unreachable!("Syscall returned number other than 0 or 1: {}", n)
    }
}

pub fn map_delete_elem(map_elem_attr: MapElemAttr) -> Result<(),Error> {
    match ebpf_syscall(Action::MapDeleteElem, map_elem_attr) {
        0 => Ok(()),
        -1 => Err(Error::last_os_error()),
        n => unreachable!("Syscall returned number other than 0 or 1: {}", n)
    }
}
