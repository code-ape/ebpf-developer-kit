
use std::io::{Error,ErrorKind};
use std::mem;
use std::marker::PhantomData;

use ::v1::map::lowlevel::{
    create_map,
    map_update_elem,
    map_lookup_elem,
    map_delete_elem,
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
    DeletableEntries,
    Iter,
    CreateResult
};

#[cfg(feature = "beta")]
pub struct HashMap<K: Clone, V: Clone> {
    map_fd: MapFd,
    max_entries: u32,
    key: PhantomData<K>,
    value: PhantomData<V>
}

impl<K: Clone,V: Clone> Map for HashMap<K,V> {
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

    fn max_entries(&self) -> u32 {
        self.max_entries
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

    fn get_next_key(&self, k: Option<K>) -> Result<Option<K>,Error> {
        let mut next_key : K = unsafe { mem::uninitialized() };
        let next_key_ptr : *mut K = &mut next_key;

        let current_key : K = match k {
            Some(x) => x,
            None => unsafe { mem::zeroed() }
        };

        let map_elem_attr = MapElemAttr {
            map_fd: unsafe { self.map_fd.get_fd() } as u32,
            key: &current_key as *const K as u64,
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

    fn iter(&self) -> Iter<Self,K,V> {
        self.into_iter()
    }
}

impl<K: Clone, V: Clone> MutableMap for HashMap<K,V> {
    fn set(&mut self, k: Self::Key, v: Self::Value) -> Result<(),Error> {
        let map_elem_attr = MapElemAttr {
            map_fd: unsafe { self.map_fd.get_fd() } as u32,
            key: &k as *const K as u64,
            value_or_next_key: &v as *const V as u64,
            flags: WriteOption::CreateOrUpdate as u64
        };
        unsafe { map_update_elem(map_elem_attr) }
    }
}

impl<K: Clone, V: Clone> DeletableEntries for HashMap<K,V> {
    fn set_if(&mut self, k: Self::Key, v: Self::Value, opt: WriteOption) -> Result<(),Error> {
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


impl<'a,K,V> IntoIterator for &'a HashMap<K,V>
    where K: Clone, V: Clone
{
    type Item = (K,V);
    type IntoIter = Iter<'a,HashMap<K,V>,K,V>;

    fn into_iter(self) -> Self::IntoIter {
        Iter::new(self)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    
    #[test]
    fn hashmap_iterator() {
        let mut hm = HashMap::new(128).unwrap();
        hm.set_if(1,51, WriteOption::CreateEntry).unwrap();
        hm.set_if(80,74, WriteOption::CreateEntry).unwrap();
        hm.set_if(17,4, WriteOption::CreateEntry).unwrap();

        let mut expected = HashSet::new();
        expected.insert((1,51));
        expected.insert((80,74));
        expected.insert((17,4));

        let mut found = HashSet::new();
        for (k,v) in hm.iter() {
            found.insert((k,v));
        }
        assert_eq!(expected, found);
    }
}