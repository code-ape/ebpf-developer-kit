

use std::io::Error;

use ::v1::lowlevel::{
    MapCreateAttr,
    MapElemAttr,
    MapFd,
    ebpf_syscall,
    Action
};


pub unsafe fn create_map(map_create_attr: MapCreateAttr) -> Result<MapFd,Error> {
    match ebpf_syscall(Action::MapCreate, map_create_attr) {
        n if n > 0 => Ok(MapFd::new(n as i32)),
        -1 => Err(Error::last_os_error()),
        n => unreachable!("Syscall returned number other than 0 or 1: {}", n)
    }
}


pub unsafe fn map_update_elem(map_elem_attr: MapElemAttr) -> Result<(),Error> {
    match ebpf_syscall(Action::MapUpdateElem, map_elem_attr) {
        0 => Ok(()),
        -1 => Err(Error::last_os_error()),
        n => unreachable!("Syscall returned number other than 0 or 1: {}", n)
    }
}


pub unsafe fn map_lookup_elem(map_elem_attr: MapElemAttr) -> Result<(),Error> {
    match ebpf_syscall(Action::MapLookupElem, map_elem_attr) {
        0 => Ok(()),
        -1 => Err(Error::last_os_error()),
        n => unreachable!("Syscall returned number other than 0 or 1: {}", n)
    }
}

pub unsafe fn map_delete_elem(map_elem_attr: MapElemAttr) -> Result<(),Error> {
    match ebpf_syscall(Action::MapDeleteElem, map_elem_attr) {
        0 => Ok(()),
        -1 => Err(Error::last_os_error()),
        n => unreachable!("Syscall returned number other than 0 or 1: {}", n)
    }
}

pub unsafe fn map_get_next_key(map_elem_attr: MapElemAttr) -> Result<(),Error> {
    match ebpf_syscall(Action::MapGetNextKey, map_elem_attr) {
        0 => Ok(()),
        -1 => Err(Error::last_os_error()),
        n => unreachable!("Syscall returned number other than 0 or 1: {}", n)
    }
}
