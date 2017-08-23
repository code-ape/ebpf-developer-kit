
//! Create and manipulate eBPF maps.
//!
//! # Examples
//!
//! ```
//! use ebpf::v1::map::{
//!     HashMap as EbpfHashMap,
//!     WriteOption,
//!     Map,
//!     MutableMap,
//!     DeletableEntries,
//! };
//!
//! fn main() {
//! 
//!     println!("Creating eBPF map (type 'hashmap') to hold 512 entries.");
//!     let mut hm = EbpfHashMap::new(512).expect("Hashmap creation failed!");
//! 
//!     println!("Setting key 1 to value 101.");
//!     hm.set_if(1,101, WriteOption::CreateEntry).expect("Write failed!");
//!     println!("Getting value for key 1.");
//!     let v = hm.get(1).expect("Read failed!");
//! 
//!     println!("Asserting value retrieved equals 101.");
//!     assert_eq!(v, 101);
//! 
//!     println!("Deleting value for key 1.");
//!     hm.delete(1).expect("Delete failed!");
//! 
//!     println!("Verifying no value for key 1.");
//!     assert!(hm.get(1).is_err());
//! }
//! ```
//!

mod core;
pub mod lowlevel;

#[cfg(feature = "kernel_3_18")]
mod array;
#[cfg(feature = "kernel_3_18")]
mod hashmap;

pub use ::v1::lowlevel::WriteOption;

pub use self::core::{
    // traits
    Map,
    MutableMap,
    DeletableEntries,
    Iter,
};

#[cfg(feature = "kernel_3_18")]
pub use self::array::Array;
#[cfg(feature = "kernel_3_18")]
pub use self::hashmap::HashMap;
