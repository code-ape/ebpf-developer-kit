
use std::io::Error;
use std::mem;
use std::marker::PhantomData;

use ::v1::data::{
    MapCreateAttr,
    MapType,
    MapId,
    MapElemAttr
};

use ::v1::map::{
    create_map,
    map_update_elem,
    map_lookup_elem
};

pub enum CreationError {
    PermissionDenied
}

type CreateResult<T: Map> = Result<T, Error>;

pub trait Map: Sized {
    type Key;
    type Value;

    fn new(max_entries: u32) -> CreateResult<Self>;
    fn delete(self);

    fn get(&self, k: Self::Key) -> Result<Self::Value,Error>;
    // TODO: generate iterator
}

pub trait WritableMap: Map {
    fn set(&mut self, k: Self::Key, v: Self::Value, opt: WriteOption) -> Result<(),Error>;
    fn delete_entry(&mut self, k: Self::Key) -> Result<(),()>;
}

pub struct HashMap<K,V> {
    map_id: MapId,
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
        match create_map(map_create_attr) {
            Ok(map_id) => {
                Ok(HashMap {
                    map_id: map_id,
                    max_entries: max_entries,
                    key: PhantomData,
                    value: PhantomData
                })
            },
            Err(error) => Err(error)
        }
    }

    fn delete(self) {
        unimplemented!();
    }

    fn get(&self, k: K) -> Result<V,Error> {
        unsafe {
            let mut value : V = mem::uninitialized();
            let value_ptr : *mut V = &mut value;
            let map_elem_attr = MapElemAttr {
                map_fd: self.map_id.clone().into(), // TODO
                key: &k as *const K as u64,
                value_or_next_key: value_ptr as u64,
                flags: 0
            };
            match map_lookup_elem(map_elem_attr) {
                Ok(_) => Ok(value),
                Err(error) => Err(error)
            }
        }
    }
}

pub enum WriteOption {
    CreateOrUpdate,
    CreateEntry,
    UpdateEntry,
}

impl<K,V> WritableMap for HashMap<K,V> {
    fn set(&mut self, k: Self::Key, v: Self::Value, opt: WriteOption) -> Result<(),Error> {
        let map_elem_attr = MapElemAttr {
            map_fd: self.map_id.clone().into(), // TODO
            key: &k as *const K as u64,
            value_or_next_key: &v as *const V as u64,
            flags: opt as u64
        };
        map_update_elem(map_elem_attr)
    }

    fn delete_entry(&mut self, k: Self::Key) -> Result<(),()> {
        unimplemented!();
    }
}

/*
pub struct Array<K,V> {

}

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