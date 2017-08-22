
//! Lowlevel wrappers around syscall, mostly unsafe!


use libc::{
    c_int,
    c_long,
    __u32,
    __u64,
    SYS_bpf,
    syscall as linux_syscall,
};

use std::mem;
use std::slice;
use std::fmt;
use std::io::Error;
use std::os::unix::io::RawFd;


/// Type alias to match use of a type named `__aligned_u64` in the Linux Kernel
/// code.
#[allow(non_camel_case_types)]
pub type __aligned_u64 = __u64;


/// Unsafe thin wrapper around Linux `bpf` syscall. Avoid using UNLESS you are
/// creating safe wrappers around usage of this syscall (see doc examples).
///
/// This takes an action enum and
/// a struct that implements the Attr trait then calls the syscall with: the
/// appropriate integer for it to be a `bpf` syscall, the enum cast as a c_int,
/// a pointer to the Attr struct, and the size of that struct.
///
/// Note that this is highly unsafe for two reasons:
///
/// 1. There is no enforcement of inputs and outputs.
/// For inputs there is no enforcement that the action requested (example:
/// `Action::MapCreate`) and the Attr struct given (example: `ProgLoadAttr`)
/// are correctly paired. Thus it is entirely possible to have undesired
/// behavior if the user doesn't carefully check their inputs for this
/// function. For outputs, different actions will have outputs with different
/// meanings, some could be an error code, some could be a file descriptor,
/// etc. Without the appropriate addition of types on the output they could
/// be used incorrectly.
/// 2. There is no enforcement of the validity of the Attr structs, these
/// structs have been coded to closely reflect the C union they are
/// representing. Thus some actions will write to user-space memory at the 
/// addresses of fields in some `Attr` structs (example: `MapElemAttr`).
///
/// # Examples
/// 
/// Below is a short example that will build up the two safe layers needed to
/// read a value from an eBPF HashMap.
///
/// To start with we'll address the first unsafe concern listed above, the
/// lack of enforcement that the given `Action` and `Attr` struct are correctly
/// paired, by only taking the appropriate struct, `MapElemAttr`, and
/// hardcoding the appropriate action, `Action::MapLookupElem`.
/// Also note that a `Result<(), std::io::Error>` is returned.
/// Because this function has no way of knowing the validity of the
/// MapElemAttr struct passed to it, this function is unsafe.
///
/// ```
/// // Taken from ebpf::v1::map::mod.rs;
/// use std::io::Error;
/// use ebpf::v1::lowlevel::{MapElemAttr, MapFd, ebpf_syscall, Action};
/// 
/// pub unsafe fn map_lookup_elem(map_elem_attr: MapElemAttr) -> Result<(),Error> {
///     match ebpf_syscall(Action::MapLookupElem, map_elem_attr) {
///         0 => Ok(()),
///         -1 => Err(Error::last_os_error()),
///         n => unreachable!("Syscall returned number other than 0 or 1: {}", n)
///     }
/// }
/// ```
/// 
/// And now we can address the second concern, the safety of the struct
/// `MapElemAttr`. Here we'll make a `HashMap` struct and implement a function
/// `get` which will retrieve a value from it. By enclosing the creation of the
/// `MapElemAttr` inside this function we, as the authors of this code, take
/// responsibility for the safety of it as we are invoking an unsafe call on
/// user-space pointers. 
///
/// ```
/// use std::io::Error;
/// use std::mem;
/// use std::marker::PhantomData;
/// use ebpf::v1::map::lowlevel::map_lookup_elem;
/// use ebpf::v1::lowlevel::{MapElemAttr, MapFd, Action};
///
/// struct HashMap<K,V> {
///     map_fd: MapFd,
///     max_entries: u32,
///     key: PhantomData<K>,
///     value: PhantomData<V>
/// }
///
/// fn get<K,V>(hashmap: &HashMap<K,V>, k: K) -> Result<V,Error> {
///     let mut value : V = unsafe { mem::uninitialized() };
///     let value_ptr : *mut V = &mut value;
///     let map_elem_attr = MapElemAttr {
///         map_fd: unsafe { hashmap.map_fd.get_fd() } as u32,
///         key: &k as *const K as u64,
///         value_or_next_key: value_ptr as u64,
///         flags: 0
///     };
///     let r = unsafe { map_lookup_elem(map_elem_attr) };
///     match r {
///         Ok(_) => Ok(value),
///         Err(error) => Err(error)
///     }
/// }
/// ```
///
pub unsafe fn ebpf_syscall<T: Attr>(action: Action, attr: T) -> c_long {
    linux_syscall(
        SYS_bpf,
        action as c_int,
        &attr as *const T,
        mem::size_of::<T>() as c_int
    )
}

/// Enum of all actions possible with Linux `bpf` syscall.
#[derive(Clone, Copy)]
pub enum Action {
    MapCreate,
    MapLookupElem,
    MapUpdateElem,
    MapDeleteElem,
    MapGetNextKey,
    ProgLoad,
    ObjPin,
    ObjGet,
    ProgAttach,
    ProgDetach
}

/// Helper function for printing binary of structs
unsafe fn any_as_u8_slice<'a, T: Sized>(p: T) -> &'a [u8] {
    slice::from_raw_parts(
        &p as *const T as *const u8,
        mem::size_of::<T>(),
    )
}

/// Takes something Sized, returns a `Vec<String>` with each entry
/// being the byte in binary, example: `[1,5]` (as a u8 array) turns into
/// the equivelent of `vec!["00000001", "00000101"]`
pub fn binary_vec_repr<T: Sized + Clone>(x: &T) -> Vec<String> {
    let u8_vec = unsafe { any_as_u8_slice(x.clone()).to_vec() };
    let mut ret_vec : Vec<String> = Vec::with_capacity(u8_vec.len());

    for byte in u8_vec.iter() {
        let mut s = String::new();
        // count number of zeros needed
        let mut counter = 0;
        for n in [255,127,63,31,15,7,3,1].iter() {
            if byte <= n {
                s.push('0');
            } else {
                s.push('1');
            }
        }
        ret_vec.push(s);
    }
    return ret_vec;
}

/// Returns strings of bytes (value of byte reading right to left) in table
/// format with given width (table reads left to right, top to bottem).
pub fn generate_binary_table<T: Sized + Clone>(x: &T, width: usize) -> String {
        let mut s = String::new();
        let mut counter = 0;
        
        for byte in binary_vec_repr(x).iter() {
            s.push_str(byte);
            counter += 1;
            if counter >= width {
                counter = 0;
                s.push('\n');
            } else {
                s.push(' ');
            }
        }

        return s;
}

impl fmt::Binary for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", generate_binary_table(&self, 4))
    }
}

/// Enum used to specify which type of map to create.
/// TODO: Have document reference explination of types.
pub enum MapType {
    Unspec,
    Hash,
    Array,
    ProgArray,
    PerfEventArray,
    PerCpuHash,
    PerCpuArray,
    StackTrace,
    CgroupArray,
    LruHash,
    LruPerCpuHash,
    LpmTrie
}

/// Enum used to specify type of eBPF program being loaded.
/// TODO: Have document reference explination of types.
pub enum ProgramType {
	Unspec,
	SocketFilter,
	Kprobe,
	SchedCls,
	SchedAct,
	Tracepoint,
	Xdp,
	PerfEvent,
	CgroupSkb,
	CgroupSock,
	LwtIn,
	LwtOut,
    LwtXmit
}

/// Enum used to specify behavior when writing to eBPF map.
pub enum WriteOption {
    /// Writes value regardless of if value exists.
    CreateOrUpdate,
    /// Writes value only if there isn't one already.
    CreateEntry,
    /// Writes value only if there is one already.
    UpdateEntry,
}

/// C struct used to create eBPF maps,
/// intentional lack of safe typing to stay true to C that it is mirroring.
#[repr(C)]
#[derive(Debug)]
pub struct MapCreateAttr {
    pub map_type: __u32,
    pub key_size: __u32,
    pub value_size: __u32,
    pub max_entries: __u32,
    pub map_flags: __u32
}

/// C struct used to manipulate eBPF maps,
/// intentional lack of safe typing to stay true to C that it is mirroring.
#[repr(C)]
#[derive(Debug)]
pub struct MapElemAttr {
    pub map_fd: __u32,
    // TODO: change key and value to key_ptr and value_ptr
    pub key: __aligned_u64,
    pub value_or_next_key: __aligned_u64,
    pub flags: __u64
}

/// C struct used to load eBPF programs,
/// intentional lack of safe typing to stay true to C that it is mirroring.
#[repr(C)]
#[derive(Debug)]
pub struct ProgLoadAttr {
    prog_type: __u32,
    insn_cnt: __u32,
    insns: __aligned_u64,
    license: __aligned_u64,
    log_level: __u32,
    log_size: __u32,
    log_buf: __aligned_u64,
    kern_version: __u32
}

/// C struct used to pin and get eBPF maps,
/// intentional lack of safe typing to stay true to C that it is mirroring.
#[repr(C)]
#[derive(Debug)]
pub struct ObjAttr {
    pathname: __aligned_u64,
    bpf_fd: __u32
}

/// C struct used to attach and detach eBPF programs from C groups,
/// intentional lack of safe typing to stay true to C that it is mirroring.
#[repr(C)]
#[derive(Debug)]
pub struct ProgDeAttachAttr {
    target_fd: __u32,
    attach_bpf_fd: __u32,
    attach_type: __u32,
    attach_flags: __u32
}

/// Trait used to designate valid structs to be passed to `ebpf_syscall`.
pub trait Attr: Sized {}

/// Convenient macro for implement Attr and binary printing for entities 
macro_rules! ImplAttr {
    ($t:ty) => (
        impl Attr for $t{}

        impl fmt::Binary for $t {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", generate_binary_table(&self, 4))
            }
        }
    )
}

// implement Attrs
ImplAttr!{MapCreateAttr}
ImplAttr!{MapElemAttr}


/// Holds file descriptor of an eBPF map.
#[derive(Debug)]
pub struct MapFd(RawFd);

impl MapFd {
    /// Casts a `RawFd` to a `MapFd`. Marked unsafe because
    /// it is the responsibility of the user to ensure the
    /// `RawFd` use does in fact refer to an eBPF map.
    pub unsafe fn new(fd: RawFd) -> MapFd { MapFd(fd) }

    /// Create a copy of the file descriptor and return it
    /// as a `RawFd`. Marked as unsafe because it is the
    /// responsibility of the user to ensure the `RawFd`
    /// is used appropriately and because there's no
    /// guarentee of lifetime validity.
    pub unsafe fn get_fd(&self) -> RawFd { self.0 }
}

/*
// TODO, implement way to turn OS rrrors into meaningful errors.

pub enum EbpfError {
    Unknown,
    NotPermitted,
    InvalidArgument,
    OutOfMemory
}
*/