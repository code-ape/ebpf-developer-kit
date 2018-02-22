
use std::io::{Error,ErrorKind};
use std::mem;
use std::marker::PhantomData;

use ::v1::map::lowlevel::{
    create_map,
    map_update_elem,
    map_lookup_elem,
    map_get_next_key
};

use ::v1::lowlevel::{
    MapCreateAttr,
    MapType,
    MapFd,
    MapElemAttr,
    WriteOption
};

use ::v1::map::core::{
    Map,
    MutableMap,
    Iter,
    //CreateResult
};

#[cfg(feature = "beta")]
pub struct Array<V> {
    map_fd: MapFd,
    max_entries: u32,
    value: PhantomData<V>
}

impl<V: Clone> Map for Array<V> {
    type Key = u32;
    type Value = V;

    fn new(max_entries: u32) -> Result<Self,Error> {
    //fn new(max_entries: u32) -> CreateResult<Self> {
        let map_create_attr = MapCreateAttr {
            map_type: MapType::Array as u32,
            key_size: mem::size_of::<Self::Key>() as u32,
            value_size: mem::size_of::<V>() as u32,
            max_entries: max_entries,
            map_flags: 0
        };
        let r = unsafe { create_map(map_create_attr) };
        match r {
            Ok(map_fd) => {
                Ok(Array {
                    map_fd: map_fd,
                    max_entries: max_entries,
                    value: PhantomData
                })
            },
            Err(error) => Err(error)
        }
    }

    fn max_entries(&self) -> u32 {
        self.max_entries
    }

    fn destroy(self) {
        unimplemented!();
    }

    fn get(&self, k: Self::Key) -> Result<V,Error> {
        let mut value : V = unsafe { mem::uninitialized() };
        let value_ptr : *mut V = &mut value;
        let map_elem_attr = MapElemAttr {
            map_fd: unsafe { self.map_fd.get_fd() } as u32,
            key: &k as *const Self::Key as u64,
            value_or_next_key: value_ptr as u64,
            flags: 0
        };
        let r = unsafe { map_lookup_elem(map_elem_attr) };
        match r {
            Ok(_) => Ok(value),
            Err(error) => Err(error)
        }
    }

    fn get_and_write_to_ptr(&self, k: Self::Key, val_ptr: &mut V) -> Result<(),Error> {
        let map_elem_attr = MapElemAttr {
            map_fd: unsafe { self.map_fd.get_fd() } as u32,
            key: &k as *const Self::Key as u64,
            value_or_next_key: val_ptr as *mut V as u64,
            flags: 0
        };
        unsafe { map_lookup_elem(map_elem_attr) }
    }

    fn get_next_key(&self, k: Option<Self::Key>) -> Result<Option<Self::Key>,Error> {
        let mut next_key : Self::Key = unsafe { mem::uninitialized() };
        let next_key_ptr : *mut Self::Key = &mut next_key;

        let current_key : Self::Key = match k {
            Some(x) => x,
            None => unsafe { mem::zeroed() }
        };

        let map_elem_attr = MapElemAttr {
            map_fd: unsafe { self.map_fd.get_fd() } as u32,
            key: &current_key as *const Self::Key as u64,
            value_or_next_key: next_key_ptr as u64,
            flags: 0
        };
        let r = unsafe { map_get_next_key(map_elem_attr) };
        match r {
            Ok(_) => Ok(Some(next_key)),
            Err(error) => match error.kind() {
               ErrorKind::NotFound => Ok(None),
               _ => Err(error)
            }
        }
    }

    fn iter(&self) -> Iter<Self,u32,V> {
        self.into_iter()
    }
}

impl<V: Clone> MutableMap for Array<V> {
    fn set(&mut self, k: Self::Key, v: Self::Value) -> Result<(),Error> {
        let map_elem_attr = MapElemAttr {
            map_fd: unsafe { self.map_fd.get_fd() } as u32,
            key: &k as *const Self::Key as u64,
            value_or_next_key: &v as *const V as u64,
            flags: WriteOption::CreateOrUpdate as u64
        };
        unsafe { map_update_elem(map_elem_attr) }
    }
}

impl<'a,V: Clone> IntoIterator for &'a Array<V> {
    type Item = (u32,V);
    type IntoIter = Iter<'a,Array<V>,u32,V>;

    fn into_iter(self) -> Self::IntoIter {
        Iter::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    
    #[test]
    fn array_iterator() {
        let mut array = Array::new(128).unwrap();
        array.set(1,51).unwrap();
        array.set(80,74).unwrap();
        array.set(17,4).unwrap();

        let mut expected = HashSet::new();
        expected.insert((1,51));
        for i in 2 .. 17 { expected.insert((i,0)); }
        expected.insert((17,4));
        for i in 18 .. 80 { expected.insert((i,0)); }
        expected.insert((80,74));
        for i in 81 .. 128 { expected.insert((i,0)); }

        let mut found = HashSet::new();
        for (k,v) in array.iter() {
            found.insert((k,v));
        }
        assert_eq!(expected, found);
    }
}