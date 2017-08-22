
use std::io::Error;

use ::v1::lowlevel::WriteOption;

use ::v1::map::hashmap::HashMap;
use ::v1::map::array::Array;

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
    type Key: Clone;
    type Value: Clone;

    /// Creates map with desired maximum number of entries.
    fn new(max_entries: u32) -> CreateResult<Self>;

    /// Gets maximum entries allowed for the map.
    fn max_entries(&self) -> u32;

    /// Destroys map.
    fn destroy(self);

    /// Gets value for given key.
    fn get(&self, k: Self::Key) -> Result<Self::Value,Error>;
    
    /// Useful for when dealing with map entries larger than usize and thus
    /// updating a value at a pointer is faster than creating a new one.
    fn get_and_write_to_ptr(&self, k: Self::Key, val_ptr: &mut Self::Value) -> Result<(),Error>;

    /// Given a `Some(Key)` it will find the next key after that one
    /// (does not check if key is in map), given `None` it will return
    /// the first key value. Will return `None` if at end of list.
    fn get_next_key(&self, k: Option<Self::Key>) -> Result<Option<Self::Key>,Error>;

    /// Generate an iterator for looping over all key value pairs.
    fn iter(&self) -> Iter<Self,Self::Key,Self::Value>;
}

/// Trait for maps where the user may set the values.
pub trait MutableMap: Map {
    fn set(&mut self, k: Self::Key, v: Self::Value) -> Result<(),Error>;
}

/// Trait for maps where entries can be deleted, this includes the `set_if`
/// funciton which allows you to select whether to set a value based on if
/// it current has a value. 
pub trait DeletableEntries: MutableMap {
    fn set_if(&mut self, k: Self::Key, v: Self::Value, opt: WriteOption) -> Result<(),Error>;
    fn delete(&mut self, k: Self::Key) -> Result<(),Error>;
}

/// Iterator for eBPF maps
pub struct Iter<'a,M,K,V>
    where M: Map<Key=K,Value=V> + 'a,
          K: Clone, V: Clone
{
    map: &'a M,
    last_key: Option<K>,
}

impl<'a,K,V> IntoIterator for &'a HashMap<K,V>
    where K: Clone, V: Clone
{
    type Item = (K,V);
    type IntoIter = Iter<'a,HashMap<K,V>,K,V>;

    fn into_iter(self) -> Self::IntoIter {
        Iter { map: self, last_key: None }
    }
}

impl<'a,V: Clone> IntoIterator for &'a Array<V> {
    type Item = (u32,V);
    type IntoIter = Iter<'a,Array<V>,u32,V>;

    fn into_iter(self) -> Self::IntoIter {
        Iter { map: self, last_key: None }
    }
}

impl<'a,M,K,V> Iterator for Iter<'a,M,K,V>
    where M: Map<Key=K,Value=V> + 'a,
          K: Clone, V: Clone
{
    type Item = (K,V);
    fn next(&mut self) -> Option<(K,V)> {
        match self.map.get_next_key(self.last_key.clone()).unwrap() {
            Some(next_key) => {
                let next_value = self.map.get(next_key.clone()).unwrap();
                self.last_key = Some(next_key.clone());
                return Some((next_key, next_value))
            },
            None => None       
        }
    }
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