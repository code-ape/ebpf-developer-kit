
use std::io::Error;
use std::mem;
use std::marker::PhantomData;
use std::iter;

use ::v1::lowlevel::{
    MapCreateAttr,
    MapType,
    MapFd,
    MapElemAttr,
    WriteOption
};

use ::v1::map::lowlevel::{
    create_map,
    map_update_elem,
    map_lookup_elem,
    map_delete_elem
};

/* 
// TODO: how to handle errors in a helpful way
pub enum CreationError {
    PermissionDenied
}
*/

pub type CreateResult<T: Map> = Result<T, Error>;

/// Trait for all eBPF maps, allows creation of the map, reading values from
/// it, iterating over it, and destroying it. 
pub trait Map: Sized {
    type Key;
    type Value;

    /// Creates map with desired maximum number of entries.
    fn new(max_entries: u32) -> CreateResult<Self>;

    /// Destroys map.
    fn destroy(self);

    /// Gets value for given key.
    fn get(&self, k: Self::Key) -> Result<Self::Value,Error>;
    
    /// Useful for when dealing with map entries larger than usize and thus
    /// updating a value at a pointer is faster than creating a new one.
    fn get_and_write_to_ptr(&self, k: Self::Key, val_ptr: &mut Self::Value) -> Result<(),Error>;

    /// Generate an iterator for looping over all key value pairs.
    fn iter(&self) -> Iter<(Self::Key,Self::Value)>;
}

pub trait WritableMap: Map {
    fn set(&mut self, k: Self::Key, v: Self::Value, opt: WriteOption) -> Result<(),Error>;
    fn delete(&mut self, k: Self::Key) -> Result<(),Error>;
}

/// Iterator for eBPF maps
pub struct Iter<T> {
    value: PhantomData<T>
}


pub struct HashMap<K,V> {
    map_fd: MapFd,
    max_entries: u32,
    key: PhantomData<K>,
    value: PhantomData<V>
}

impl<K,V> Map for HashMap<K,V> {
    type Key = K;
    type Value = V;

    fn new(max_entries: u32) -> CreateResult<Self> {
        let map_create_attr = MapCreateAttr {
            map_type: MapType::Hash as u32,
            key_size: mem::size_of::<K>() as u32,
            value_size: mem::size_of::<V>() as u32,
            max_entries: max_entries,
            map_flags: 0
        };
        let r = unsafe { create_map(map_create_attr) };
        match r {
            Ok(map_fd) => {
                Ok(HashMap {
                    map_fd: map_fd,
                    max_entries: max_entries,
                    key: PhantomData,
                    value: PhantomData
                })
            },
            Err(error) => Err(error)
        }
    }

    fn destroy(self) {
        unimplemented!();
    }

    fn get(&self, k: K) -> Result<V,Error> {
        let mut value : V = unsafe { mem::uninitialized() };
        let value_ptr : *mut V = &mut value;
        let map_elem_attr = MapElemAttr {
            map_fd: unsafe { self.map_fd.get_fd() } as u32,
            key: &k as *const K as u64,
            value_or_next_key: value_ptr as u64,
            flags: 0
        };
        let r = unsafe { map_lookup_elem(map_elem_attr) };
        match r {
            Ok(_) => Ok(value),
            Err(error) => Err(error)
        }
    }

    fn get_and_write_to_ptr(&self, k: K, val_ptr: &mut V) -> Result<(),Error> {
        let map_elem_attr = MapElemAttr {
            map_fd: unsafe { self.map_fd.get_fd() } as u32,
            key: &k as *const K as u64,
            value_or_next_key: val_ptr as *mut V as u64,
            flags: 0
        };
        unsafe { map_lookup_elem(map_elem_attr) }
    }

    fn iter(&self) -> Iter<(K,V)> {
        unimplemented!();
    }
}

impl<K,V> WritableMap for HashMap<K,V> {
    fn set(&mut self, k: Self::Key, v: Self::Value, opt: WriteOption) -> Result<(),Error> {
        let map_elem_attr = MapElemAttr {
            map_fd: unsafe { self.map_fd.get_fd() } as u32,
            key: &k as *const K as u64,
            value_or_next_key: &v as *const V as u64,
            flags: opt as u64
        };
        unsafe { map_update_elem(map_elem_attr) }
    }

    fn delete(&mut self, k: Self::Key) -> Result<(),Error> {
        let map_elem_attr = MapElemAttr {
            map_fd: unsafe { self.map_fd.get_fd() } as u32,
            key: &k as *const K as u64,
            value_or_next_key: 0,
            flags: 0
        };
        unsafe { map_delete_elem(map_elem_attr) }
    }
}


pub struct Array<V> {
    map_fd: MapFd,
    max_entries: u32,
    value: PhantomData<V>
}

/*

pub struct ProgramArray {

}

pub struct PerfEventArray {

}

pub struct PerCpuHashMap {

}

pub struct PerCpuArray {

}

pub struct StackTrace {

}

pub struct CgroupArray {

}

pub struct LruHashMap {

}

pub struct LruPerCpuHashMap {

}

pub struct LpmTrie {

}
*/