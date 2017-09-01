

use std::os::unix::io::RawFd;

use std::io;
use std::mem;
use std::ptr;

use ::v1::program::ProgramFd;

use libc::{
    // syscalls
    socket as raw_socket,
    setsockopt as raw_setsocketopt,
    if_nametoindex as raw_if_nametoindex,
    bind as raw_bind,
    mmap as raw_mmap,
    //
    sockaddr,
    sa_family_t,
    //
    c_int,
    c_uint,
    c_short,
    c_char,
    __u16,
    c_void,
    //
    AF_PACKET,
    PF_PACKET,
    SOCK_RAW,
    SOCK_NONBLOCK,
    SOCK_CLOEXEC,
    SOL_SOCKET,
    SOL_PACKET,
    SO_ATTACH_BPF,
    // mmap variables
    MAP_FAILED,
    PROT_READ,
    PROT_WRITE,
    MAP_SHARED,
    MAP_LOCKED,
    MAP_NORESERVE,
};

type OsResult<T> = Result<T,io::Error>;

#[derive(Debug)]
#[repr(C)]
struct SockAddrLL {
    sll_family: c_short,
    sll_protocol: __u16,
    sll_ifindex: c_int,
    sll_hatype: c_short,
    sll_pkttype: c_char,
    sll_halen: c_char,
    sll_addr: c_char,
}

impl SockAddrLL {
    fn new_blank() -> SockAddrLL {
        SockAddrLL {
            sll_family: 0,
            sll_protocol: 0,
            sll_ifindex: 0,
            sll_hatype: 0,
            sll_pkttype: 0,
            sll_halen: 0,
            sll_addr: 0,  
        }
    }
}

impl Into<sockaddr> for SockAddrLL {
    fn into(self) -> sockaddr {
        unsafe { mem::transmute(self) }
    }
}

const ETH_P_ALL: c_int = 0x0003;

#[derive(Debug)]
pub struct Interface(c_uint);

impl Interface {
    unsafe fn from_fd(fd: RawFd) -> Self {
        Interface(fd as u32)
    }
    unsafe fn get_fd(&self) -> RawFd {
        self.0 as RawFd
    }
}


#[derive(Debug)]
pub struct Socket(c_uint);

impl Socket {
    unsafe fn from_fd(fd: RawFd) -> Self {
        Socket(fd as u32)
    }
    unsafe fn get_fd(&self) -> RawFd {
        self.0 as RawFd
    }
}

fn if_nametoindex(name: &str) -> Interface {
    let name_ptr = name.as_ptr() as *const i8;
    unsafe { Interface::from_fd(
        raw_if_nametoindex(name_ptr) as i32
     ) }
}

fn bind(socket: Socket, address: SockAddrLL) -> OsResult<()> {
    let bind_result : c_int = unsafe { raw_bind(
        socket.get_fd() as i32,
        &(address.into()) as *const sockaddr,
        mem::size_of::<SockAddrLL>() as u32
    ) };

    match bind_result {
        0 => Ok(()),
        -1 => return Err(io::Error::last_os_error()),
        _ => unreachable!("syscall socket returned unreachable value!")   
    }

}

pub fn attach_ebpf_filter(socket: &Socket, ebpf_prog_fd: ProgramFd) -> OsResult<()> {
    match unsafe {
        raw_setsocketopt(
            socket.get_fd(),
            SOL_SOCKET,
            SO_ATTACH_BPF,
            &ebpf_prog_fd as *const ProgramFd as *const c_void,
            mem::size_of::<ProgramFd>() as u32
        )
    } {
        0 => Ok(()),
        -1 => return Err(io::Error::last_os_error()),
        _ => unreachable!("syscall setsockopt returned unreachable value!")   
    }
}

pub fn open_raw_sock(interface_name: &str) -> OsResult<Socket> {
    let socket_result = unsafe {
        raw_socket(
            PF_PACKET,
            SOCK_RAW | SOCK_NONBLOCK | SOCK_CLOEXEC,
            ETH_P_ALL.to_be()
        )
    };

    let socket : Socket = match socket_result {
        n if n > 0 => unsafe { Socket::from_fd(n) },
        -1 => return Err(io::Error::last_os_error()),
        _ => unreachable!("syscall socket returned unreachable value!")
    };

    let mut sll = SockAddrLL::new_blank();

    sll.sll_family = AF_PACKET as i16;
	sll.sll_ifindex = unsafe { if_nametoindex(interface_name).get_fd() } as i32;
	sll.sll_protocol = ETH_P_ALL.to_be() as u16;

    println!("interface_index = {}", sll.sll_ifindex);

    Ok(socket)
}


#[derive(Debug)]
#[repr(C)]
struct TPacketReq3 {
    tp_block_size: c_uint,      // Minimal size of contiguous block
	tp_block_nr: c_uint,        // Number of blocks
	tp_frame_size: c_uint,      // Size of frame
	tp_frame_nr: c_uint,        // Total number of frames
	tp_retire_blk_tov: c_uint,  // timeout in msecs
	tp_sizeof_priv: c_uint,     // offset to private data area
	tp_feature_req_word: c_uint,
}

const PACKET_RX_RING : c_int = 5;

pub fn set_socket_rx_ring(
    socket: &Socket, block_size: c_uint, block_number: c_uint, block_timeout: c_uint
    ) -> OsResult<()> {

    let tp3 = TPacketReq3 {
        tp_block_size: block_size,
        tp_block_nr: block_number,
        tp_frame_size: block_size,
        tp_frame_nr: block_number,
        tp_retire_blk_tov: block_timeout,
        tp_sizeof_priv: 0,
        tp_feature_req_word: 0
    };

    match unsafe {
        raw_setsocketopt(
            socket.get_fd(),
            SOL_PACKET,
            PACKET_RX_RING,
            &tp3 as *const TPacketReq3 as *const c_void,
            mem::size_of::<TPacketReq3>() as u32
        )
    } {
        0 => Ok(()),
        -1 => return Err(io::Error::last_os_error()),
        _ => unreachable!("syscall setsockopt returned unreachable value!")   
    }
}

#[derive(Debug)]
pub struct PacketRing {
    address: *const c_void,
    index: usize,
    block_size: c_uint,
    block_number: c_uint
}

pub fn mmap_rx_ring(
    socket: &Socket, block_size: c_uint, block_number: c_uint, block_timeout: c_uint
    ) -> OsResult<PacketRing> {
    match unsafe {
        raw_mmap(
            // address, if null then returns the address
            ptr::null_mut(), 
            // mmap length
            (block_size * block_number) as usize,
            // protocol
            PROT_READ | PROT_WRITE,
            // flags
            MAP_SHARED | MAP_LOCKED | MAP_NORESERVE,
            // target file descriptor
            socket.get_fd(),
            // offset
            0
        )
    } {
        n if n > (0 as *mut c_void) => {
            Ok( PacketRing {
                address: n,
                index: 0,
                block_size: block_size,
                block_number: block_number
            } )
        },
        n if n == MAP_FAILED => Err(io::Error::last_os_error()),
        _ => unreachable!("syscall setsockopt returned unreachable value!")
    }
}

