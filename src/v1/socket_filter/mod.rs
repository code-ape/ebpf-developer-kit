
use std::os::unix::io::RawFd;

use std::{
    io,
    mem,
    ptr,
    rc,
    thread,
    time,
    ffi,
    //fmt,
};

use ::v1::program::{
    EbpfProgram,
    ProgramFd,
};

use ::v1::program as program;

use libc::{
    // syscalls
    setsockopt as raw_setsocketopt,
    if_nametoindex as raw_if_nametoindex,
    bind as raw_bind,
    mmap as raw_mmap,
    //
    sockaddr,
    //
    c_int,
    c_uint,
    //c_short,
    //c_char,
    //__u8,
    //__u16,
    //__u32,
    //__u64,
    c_void,
    //
    //AF_PACKET,
    //PF_PACKET,
    //SOCK_RAW,
    //SOCK_NONBLOCK,
    //SOCK_CLOEXEC,
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

use lib_networking::v1::linux as networking;

//use self::networking::packet as packet;
use self::networking::packet::{
    SockAddrLL,
    PacketOption,
    TPacketVersions,
    TPacketReq3,
    //TPacketBdTs,
    //TPacketHdrVariant1,
    //TPacket3Hdr,
    //TPacketHdrV1,
    //TPacketBlockDesc,
    TPacketBlockDescV1,
    TPacketBlockDescV1Iter
};

//use self::networking::socket as socket;
use self::networking::socket::{
    socket_syscall,
    AddressFamily,
};

//use self::networking::ether as ether;
use self::networking::ether::{
    Protocol
};

//use self::networking::net as net;
use self::networking::net::{
    SocketDescription,
    SockType
};

pub type OsResult<T> = Result<T,io::Error>;

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

pub fn if_nametoindex(name: &str) -> OsResult<Interface> {
    let name_cstr = ffi::CString::new(name)?;
    match unsafe { raw_if_nametoindex((&name_cstr).as_ptr() as *const i8) } {
        0 => Err(io::Error::last_os_error()),
        n if n > 0 => unsafe { Ok(Interface::from_fd(n as i32)) },
        _ => unreachable!("syscall if_nametoindex returned unreachable value!")   
    }
}

fn bind(socket: &Socket, address: SockAddrLL) -> OsResult<()> {
    let bind_result : c_int = unsafe { raw_bind(
        socket.get_fd() as i32,
        &(address.into()) as *const sockaddr,
        mem::size_of::<SockAddrLL>() as u32
    ) };

    match bind_result {
        0 => {
            debug!("fn bind: success = true");
            Ok(())
        },
        -1 => {
            let e = io::Error::last_os_error();
            error!("fn bind: success = false, error = {}", e);
            Err(e)
        },
        _ => unreachable!("syscall socket returned unreachable value!")   
    }

}

pub unsafe fn attach_ebpf_filter_raw(socket: &Socket, ebpf_prog_fd: ProgramFd) -> OsResult<()> {
    match raw_setsocketopt(
        socket.get_fd(),
        SOL_SOCKET,
        SO_ATTACH_BPF,
        &ebpf_prog_fd as *const ProgramFd as *const c_void,
        mem::size_of::<ProgramFd>() as u32
    ) {
        0 => Ok(()),
        -1 => return Err(io::Error::last_os_error()),
        _ => unreachable!("syscall setsockopt returned unreachable value!")   
    }
}

pub fn attach_ebpf_filter<'a,T>(socket: &Socket, sf_prog: T) -> OsResult<()> 
    where T: AsRef<program::SocketFilter<'a>>
{
    unsafe { attach_ebpf_filter_raw(socket, sf_prog.as_ref().file_descriptor()) }
}

pub fn open_raw_sock() -> OsResult<Socket> {
    match socket_syscall(
        AddressFamily::Packet,
        SocketDescription {
            sock_type: SockType::Raw,
            nonblock: true,
            close_on_exec: true
        },
        Protocol::All
    ) {
        Ok(n) => {
            let s = unsafe { Socket::from_fd(n) };
            debug!("fn open_raw_sock, success = false, socket = {:?}", s);
            Ok(s)
        },
        Err(e) => {
            error!("fn open_raw_sock, success = false, error = {}", e);
            Err(e)
        }
    }
}

pub fn bind_to_interface(socket: &Socket, interface_name: &str) -> OsResult<()> {

    let interface_index = if_nametoindex(interface_name)?;

    debug!("fn bind_to_interface, interface_index = {:?}, interface_name = {}", interface_index, interface_name);

    let sll = SockAddrLL {
        sll_family: AddressFamily::Packet as i16,
        sll_protocol: Protocol::All as u16,
        sll_ifindex: unsafe { interface_index.get_fd() },
        sll_hatype: 0,
        sll_pkttype: 0,
        sll_halen: 0,
        sll_addr: [0,0,0,0,0,0,0,0]
    };

    trace!("fn bind_to_interface, SockAddrLL = {:?}", sll);

    bind(socket, sll)
}

pub fn set_packet_version_v3(socket: &Socket) -> OsResult<()> {
    match unsafe {
        raw_setsocketopt(
            socket.get_fd(),
            SOL_PACKET,
            PacketOption::Version as i32,
            &(TPacketVersions::TPACKET_V3 as i32) as *const c_int as *const c_void,
            mem::size_of::<c_int>() as u32
        )
    } {
        0 => {
            debug!("fn set_packet_version_v3, success = true");
            Ok(())
        },
        -1 => {
            let e = io::Error::last_os_error();
            error!("fn set_packet_version_v3, success = false, error = {}", e);
            Err(e)
        },
        _ => unreachable!("syscall setsockopt returned unreachable value!")   
    }
}

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
            PacketOption::RxRing as i32,
            &tp3 as *const TPacketReq3 as *const c_void,
            mem::size_of::<TPacketReq3>() as u32
        )
    } {
        0 => {
            debug!("fn set_socket_rx_ring, success = true");
            Ok(())
        },
        -1 => {
            let e = io::Error::last_os_error();
            error!("fn set_socket_rx_ring, success = false, error = {}", e);
            Err(e)
        },
        _ => unreachable!("syscall setsockopt returned unreachable value!")   
    }
}

#[derive(Debug)]
struct BlockRaw {//<'a> {
    //packet_ring: &'a PacketRing<'a>,
    index: c_uint,
}


#[derive(Debug)]
pub struct Block(rc::Rc<BlockRaw>);
//pub struct Block<'a>(rc::Rc<BlockRaw<'a>>);

//impl<'a> Block<'a> {
impl Block {
    //fn new(packet_ring: &'a PacketRing, index: c_uint) -> Block<'a> {
    fn new(index: c_uint) -> Block {
        Block(
            rc::Rc::new( BlockRaw {
                //packet_ring: packet_ring,
                index: index
            } )
        )
    }

    fn is_ready(&self, packet_ring: &PacketRing) -> bool {
        let sc = rc::Rc::strong_count(&self.0);
        let wc = rc::Rc::weak_count(&self.0);
        let b_desc = self.block_desc(packet_ring);
        let bs = b_desc.hdr.block_status;
        let result = (sc == 1) && (wc == 0) && (bs != 0);

        //println!("Block({}).is_ready() => wc({}), sc({}), bs({}), result({})",
        //    self.0.index, wc, sc, bs, result);
        //println!("Block(0).desc = {:?}\n",b_desc);
        
        result
    }

    fn block_desc(&self, packet_ring: &PacketRing) -> &mut TPacketBlockDescV1 {
        let block_desc : &mut TPacketBlockDescV1 = unsafe {
            let ptr = (packet_ring.address as u64) + 
                (packet_ring.block_size as u64 * self.0.index as u64);
            // TODO
            let p2 = mem::transmute::<u64, &mut TPacketBlockDescV1>(ptr as u64);
            // TODO
            println!("p2 = {:?}", p2);
            assert!(p2.version == TPacketVersions::TPACKET_V3);
            p2
        };
        block_desc
    }

    pub fn iter(&self, packet_ring: &PacketRing) -> TPacketBlockDescV1Iter {
        println!("Block({}) constructing iterator...", self.0.index);
        let block_desc = self.block_desc(packet_ring);
        block_desc.iter()
    }

}

#[derive(Debug)]
pub struct PacketRing {
//pub struct PacketRing<'a> {
    address: *const c_void,
    index: c_uint,
    block_size: c_uint,
    block_number: c_uint,
    ///blocks: Vec<Block<'a>>
    blocks: Vec<Block>
}

//impl<'a> PacketRing<'a> {
impl PacketRing {
    pub fn get_next(&mut self) -> (&Block, &PacketRing) {
        let i = self.index;
        let block = &self.blocks[i as usize];
        let mut ready = block.is_ready(self);

        while !ready {
            //println!("Sleeping 0.5 seconds ...");
            thread::sleep(time::Duration::from_millis(500));
            ready = block.is_ready(self);
        }
        self.index = (i + 1) % self.block_number;
        (&block, self)
    }

    pub fn is_ready(&self) -> bool {
        self.blocks[self.index as usize].is_ready(self)
    }
}

pub fn mmap_rx_ring(
    socket: &Socket, block_size: c_uint, block_number: c_uint
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
            let mut pr = PacketRing {
                address: n,
                index: 0,
                block_size: block_size,
                block_number: block_number,
                blocks: Vec::with_capacity(block_number as usize)
            };

            for e in 0..block_number {
                let b = Block::new(e);
                pr.blocks.push(b);
            }
            debug!("fn mmap_rx_ring, success = true");
            trace!("fn mmap_rx_ring, PacketRing = {:?}", pr);
            Ok(pr)
        },
        n if n == MAP_FAILED => {
            let e = io::Error::last_os_error();
            error!("fn mmap_rx_ring, success = false, error = {}", e);
            Err(e)
        },
        _ => unreachable!("syscall mmap returned unreachable value!")
    }
}


pub enum ReadMethod {
    RxRing {
        block_size: c_uint,
        block_count: c_uint,
        ring_timeout: c_uint
    }
}

//pub struct Info<'a> {
//    pub interface: &'a str,
//    pub packet_version: PacketVersion,
//    pub read_method: ReadMethod,
//    pub filter_program: program::SocketFilter<'a>,
//}