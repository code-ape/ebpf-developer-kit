
use libc::{c_int, c_long, __u32, __u64};
use std::os::unix::io::RawFd;
use std::mem;
use std::fmt;
use std::io::{
    Error,
};

type __aligned_u64 = __u64;

#[derive(Clone)]
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

const ACTION_SIZE: usize = mem::size_of::<Action>();


impl fmt::Binary for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v = unsafe {
            mem::transmute::<Action, [u8; ACTION_SIZE]>((*self).clone()).to_vec()
        };

        let mut r: fmt::Result = Result::Ok(());
        let mut counter = 1;
        
        for byte in v.iter() {
            let mut padding = "".to_string();
            for n in [255,127,63,31,15,7,3,1].iter() {
                if byte <= n {
                    padding += "0";
                } else {
                    break;
                }
            }
            if counter >= 4 {
                counter = 1;
                r = write!(f, "{}{:b}\n", padding, byte);
            } else {
                counter += 1;
                r = write!(f, "{}{:b} ", padding, byte);
            }
            if r.is_err() {
                return r;
            }
        }

        return r;
    }
}

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

trait BpfAttr {
    /// Implemented on a struct it returns the byte array for
    /// the struct and it's size
    fn into_byte_vec(self) -> Vec<u8>;
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct MapCreateAttr {
    pub map_type: __u32,
    pub key_size: __u32,
    pub value_size: __u32,
    pub max_entries: __u32,
    pub map_flags: __u32
}

const MAP_CREATE_ATTR_SIZE: usize = mem::size_of::<MapCreateAttr>();

impl BpfAttr for MapCreateAttr {
    fn into_byte_vec(self) -> Vec<u8> {
        return unsafe {
            let s = mem::transmute::<MapCreateAttr, [u8; MAP_CREATE_ATTR_SIZE]>(self);
            s.to_vec()
        }
    }
}

impl fmt::Binary for MapCreateAttr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v = self.clone().into_byte_vec();

        let mut r: fmt::Result = Result::Ok(());
        let mut counter = 1;
        
        for byte in v.iter() {
            let mut padding = "".to_string();
            for n in [255,127,63,31,15,7,3,1].iter() {
                if byte <= n {
                    padding += "0";
                } else {
                    break;
                }
            }
            if counter >= 4 {
                counter = 1;
                r = write!(f, "{}{:b}\n", padding, byte);
            } else {
                counter += 1;
                r = write!(f, "{}{:b} ", padding, byte);
            }
            if r.is_err() {
                return r;
            }
        }

        return r;
    }
}

#[repr(C)]
pub struct MapElemAttr {
    pub map_fd: __u32,
    // TODO: change key and value to key_ptr and value_ptr
    pub key: __aligned_u64,
    pub value_or_next_key: __aligned_u64,
    pub flags: __u64
}

#[repr(C)]
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

#[repr(C)]
pub struct ObjAttr {
    pathname: __aligned_u64,
    bpf_fd: __u32
}

#[repr(C)]
pub struct ProgDeAttachAttr {
    target_fd: __u32,
    attach_bpf_fd: __u32,
    attach_type: __u32,
    attach_flags: __u32
}

pub trait Attr: Sized {}

impl Attr for MapCreateAttr {}

impl Attr for MapElemAttr {}

/*
pub union Attr {
    pub map_create: MapCreateAttr,
    pub map_elem: MapElemAttr,
    prog_load: ProgLoadAttr,
    obj: ObjAttr,
    prog_de_attach: ProgDeAttachAttr
} // __attribute__((aligned(8)))
*/

#[derive(Debug, Clone)]
pub struct MapId(c_long);

impl MapId {
    pub fn new(n: c_long) -> MapId {
        MapId(n)
    }
}

impl Into<__u32> for MapId {
    fn into(self) -> __u32 {
        self.0 as __u32
    }
}

pub enum EbpfError {
    Unknown,
    NotPermitted,
    InvalidArgument,
    OutOfMemory
}


struct RawOsError(i32);

impl RawOsError {
    #[inline(always)]
    fn new(n: i32) -> RawOsError {
        RawOsError(n)
    }

    fn get_last() -> RawOsError {
        let error = Error::last_os_error();
        let raw_error = error.raw_os_error().expect("Failed to get raw OS error!");
        RawOsError::new(raw_error)
    }
}

pub type MapCreateResult = Result<MapId,RawOsError>;

//pub type MapItemGetResult = Result<,RawOsError>;

//pub type MapItemSetResult = Result<,RawOsError>;

