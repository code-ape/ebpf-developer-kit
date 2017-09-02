

use std::os::unix::io::RawFd;

use std::{
    io,
    mem,
    ptr,
    rc,
    thread,
    time,
    ffi,
    fmt,
};


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
    //
    c_int,
    c_uint,
    c_short,
    c_char,
    __u8,
    __u16,
    __u32,
    __u64,
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
pub struct SockAddrLL {
    sll_family: c_short,
    sll_protocol: __u16,
    sll_ifindex: c_int,
    sll_hatype: c_short,
    sll_pkttype: c_char,
    sll_halen: c_char,
    sll_addr: c_char,
    padding: u32 // add 4 bytes to bring to 20 total
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
            padding: 0
        }
    }
}

impl Into<sockaddr> for SockAddrLL {
    fn into(mut self) -> sockaddr {
        self.sll_family = self.sll_family;
        self.sll_protocol = self.sll_protocol.to_be();
        self.sll_ifindex = self.sll_ifindex;//.to_be();
        unsafe { 
            let p = &self as *const SockAddrLL;
            *(mem::transmute::<*const SockAddrLL, *const sockaddr>(p))
         }
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

pub fn if_nametoindex(name: &str) -> OsResult<Interface> {
    let name_cstr = ffi::CString::new(name).unwrap();
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

pub fn open_raw_sock() -> OsResult<Socket> {
    match unsafe {
        raw_socket(
            PF_PACKET,
            SOCK_RAW | SOCK_NONBLOCK | SOCK_CLOEXEC,
            ETH_P_ALL.to_be()
        )
    } {
        n if n > 0 => unsafe { Ok(Socket::from_fd(n)) },
        -1 => Err(io::Error::last_os_error()),
        _ => unreachable!("syscall socket returned unreachable value!")
    }
}

pub fn bind_to_interface(socket: &Socket, interface_name: &str) -> OsResult<()> {

    let interface_index = if_nametoindex(interface_name)?;

    println!("interface_index = {:?}", interface_index);

    let mut sll = SockAddrLL::new_blank();

    sll.sll_family = AF_PACKET as i16;
	sll.sll_protocol = ETH_P_ALL as u16;
	sll.sll_ifindex = 2; //unsafe { interface_index.get_fd() };

    println!("sll = {:?}", sll);

    bind(socket, sll)
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
const TPACKET_V3 : c_int = 2;
const PACKET_VERSION : c_int = 10;

pub fn set_packet_version_v3(socket: &Socket) -> OsResult<()> {
    match unsafe {
        raw_setsocketopt(
            socket.get_fd(),
            SOL_PACKET,
            PACKET_VERSION,
            &TPACKET_V3 as *const c_int as *const c_void,
            mem::size_of::<c_int>() as u32
        )
    } {
        0 => Ok(()),
        -1 => return Err(io::Error::last_os_error()),
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
#[repr(C)]
struct TPacketBdTs {
    ts_sec: c_uint,
    ts_usec_or_nsec: c_uint
}

#[derive(Debug)]
#[repr(C)]
struct TPacketHdrVariant1 {
    tp_rxhash: __u32,
    tp_vlan_tci: __u32,
    tp_vlan_tpid: __u16,
    tp_padding: __u16
}

#[derive(Debug)]
#[repr(C)]
pub struct TPacket3Hdr {
    tp_next_offset: __u32,
    tp_sec: __u32,
    tp_nsec: __u32,
    tp_snaplen: __u32,
    tp_len: __u32,
    tp_status: __u32,
    tp_mac: __u16,
    tp_net: __u16,
    tp_hdr_variant_1: TPacketHdrVariant1,
    tp_padding: __u8
}

#[derive(Debug)]
#[repr(packed)]
struct MacAddress([u8;6]);

impl fmt::Display for MacAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:X}:{:X}:{:X}:{:X}:{:X}:{:X}",
            self.0[0], self.0[1], self.0[2], self.0[3],
            self.0[4], self.0[5]
        )
    }    
}

#[derive(Debug)]
#[repr(packed)]
pub struct EthHdr {
    h_dest: MacAddress,
    h_source: MacAddress,
    h_proto: __u16
}

impl fmt::Display for EthHdr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Destination: {}, Source: {}, Proto: {:X}",
            self.h_dest, self.h_source, self.h_proto
        )
    }    
}

impl TPacket3Hdr {
    pub fn get_eth_hdr(&self) -> &EthHdr {
        unsafe {
            &(*((
                (self as *const TPacket3Hdr as u64) + 
                (self.tp_mac as u64)
            ) as *const EthHdr))
        }
    }
}

impl fmt::Display for TPacket3Hdr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EthHdr:: {}",
            self.get_eth_hdr()
        )
    }
}

#[derive(Debug)]
#[repr(C)]
struct TPacketHdrV1 {
    block_status: __u32,
    num_pkts: __u32,
    offset_to_first_pkt: __u32,
    blk_len: __u32,
    seq_num: __u64,
    t_packet_bd_ts: TPacketBdTs
}

#[derive(Debug)]
#[repr(C)]
struct TPacketBlockDesc {
    version: __u32,
    offset_to_priv: __u32,
    hdr: TPacketHdrV1
}

#[derive(Debug)]
struct BlockRaw {//<'a> {
    //packet_ring: &'a PacketRing<'a>,
    index: c_uint,
}

#[derive(Debug)]
pub struct PacketIter<'a> {
    block_desc: &'a mut TPacketBlockDesc,
    last_index: isize,
    next_offset: u32
}

impl TPacketBlockDesc {
    pub fn iter(&mut self) -> PacketIter {
        println!("TPacketBlockDesc with {} packets constructing iterator...",
            self.hdr.num_pkts);
        PacketIter {
            block_desc: self,
            last_index: -1,
            next_offset: 0
        }
    }
}

impl<'a> Iterator for PacketIter<'a> {
    type Item = &'a TPacket3Hdr;
    fn next(&mut self) -> Option<&'a TPacket3Hdr> {
        match self.last_index + 1 {
            n if n < (self.block_desc.hdr.num_pkts as isize) => unsafe {
                let block_desc_ptr = self.block_desc as *const TPacketBlockDesc;
                let addr_ptr : *const c_void = (
                        (self.block_desc.hdr.offset_to_first_pkt as u64) + 
                        (block_desc_ptr as u64) +
                        (self.next_offset as u64)
                    ) as *const c_void;
                let tp3hdr = mem::transmute::<*const c_void, &TPacket3Hdr>(addr_ptr);
                self.last_index = self.last_index + 1;
                self.next_offset = self.next_offset + tp3hdr.tp_next_offset;
                return Some(tp3hdr);
            },
            _ => {
                self.block_desc.hdr.block_status = 0;
                None
            }
        }
    }
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

    fn block_desc(&self, packet_ring: &PacketRing) -> &mut TPacketBlockDesc {
        let block_desc : &mut TPacketBlockDesc = unsafe {
            let ptr = (packet_ring.address as u64) + 
                (packet_ring.block_size as u64 * self.0.index as u64);
            let p2 = mem::transmute(ptr as u64);
            p2
        };
        block_desc
    }

    pub fn iter(&self, packet_ring: &PacketRing) -> PacketIter {
        println!("Block({}) constructing iterator...", self.0.index);
        let mut block_desc = self.block_desc(packet_ring);
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
            Ok(pr)
        },
        n if n == MAP_FAILED => Err(io::Error::last_os_error()),
        _ => unreachable!("syscall setsockopt returned unreachable value!")
    }
}

