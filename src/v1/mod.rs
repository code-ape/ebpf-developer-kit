
use std::os::unix::io::RawFd;

// use libc::{setsockopt, SOL_SOCKET};
use libc::{syscall, c_long, __u32, __u64, SYS_bpf};

type __aligned_u64 = __u64;

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

#[repr(C)]
struct MapCreateAttr {
    map_type: __u32,
    key_size: __u32,
    value_size: __u32,
    max_entries: __u32,
    map_flags: __u32
}

#[repr(C)]
struct MapElemAttr {
    map_fd: __u32,
    key: __aligned_u64,
    value_or_next_key: __aligned_u64,
    flags: __u64
}

#[repr(C)]
struct ProgLoadAttr {
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
struct ObjAttr {
    pathname: __aligned_u64,
    bpf_fd: __u32
}

#[repr(C)]
struct ProgDeAttachAttr {
    target_fd: __u32,
    attach_bpf_fd: __u32,
    attach_type: __u32,
    attach_flags: __u32
}

union Attr {
    map_create: MapCreateAttr,
    map_elem: MapElemAttr,
    prog_load: ProgLoadAttr,
    obj: ObjAttr,
    prog_de_attach: ProgDeAttachAttr
} // __attribute__((aligned(8)))

pub fn create_map(map_type: MapType, key_size: usize, value_size: usize,
                  max_entries: usize) -> RawFd {
    
    unimplemented!();
}

pub fn bpf_syscall(action: Action) -> c_long {
    return syscall(SYS_bpf, );
    unimplemented!();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
