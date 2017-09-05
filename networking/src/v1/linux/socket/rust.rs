
use std::io;

use libc::{
    c_int,
    c_uint,
    socket as libc_socket
};

use ::v1::linux::socket::c as c;
use ::v1::linux::net as net;
use ::v1::linux::ether as ether;

pub enum AddressFamily {
    /// Ensures that zero is not a valid option
    Unspec = c::AF_UNSPEC as isize,
    /// Unix domain socket
    Unix = c::AF_UNIX as isize,
    /// Internet IP Protocol
    Ip = c::AF_INET as isize,
    /// Amateur Radio AX.25
    Ax25 = c::AF_AX25 as isize,
    /// Novell IPX
    Ipx = c::AF_IPX as isize,
    /// AppleTalk DDP 
    AppleTalk = c::AF_APPLETALK as isize,
    /// Amateur Radio NET/ROM 
    NetRom = c::AF_NETROM as isize,
    /// Multiprotocol bridge 
    Bridge = c::AF_BRIDGE as isize,
    /// ATM PVCs
    AtmPvc = c::AF_ATMPVC as isize,
    /// Reserved for X.25 project 
    X25 = c::AF_X25 as isize,
    /// IP version 6
    Ipv6 = c::AF_INET6 as isize,
    /// Amateur Radio X.25 PLP
    Rose = c::AF_ROSE as isize,
    /// Reserved for DECnet project
    DecNet = c::AF_DECnet as isize,
    /// Reserved for 802.2LLC project
    NetBeui = c::AF_NETBEUI as isize,
    /// Security callback pseudo AF
    Security = c::AF_SECURITY as isize,
    /// PF_KEY key management API
    Key = c::AF_KEY as isize,
    Netlink = c::AF_NETLINK as isize,
    /// Packet family
    Packet = c::AF_PACKET as isize,
    /// Ash
    Ash = c::AF_ASH as isize,
    /// Acorn Econet
    Econet = c::AF_ECONET as isize,
    /// ATM SVCs
    AtmSvc = c::AF_ATMSVC as isize,
    /// RDS sockets 
    Rds = c::AF_RDS as isize,
    /// Linux SNA Project (nutters!) */
    Sna = c::AF_SNA as isize,
    /// IRDA sockets
    Irda = c::AF_IRDA as isize,
    /// PPPoX sockets
    PPPox = c::AF_PPPOX as isize,
    /// Wanpipe API Sockets
    Wanpipe = c::AF_WANPIPE as isize,
    /// Linux LLC
    Llc = c::AF_LLC as isize,
    /// Native InfiniBand address
    InfiniBand = c::AF_IB as isize,
    /// MPLS
    Mpls = c::AF_MPLS as isize,
    /// Controller Area Network
    Can = c::AF_CAN as isize,
    /// TIPC sockets
    Tipc = c::AF_TIPC as isize,
    /// Bluetooth sockets
    Bluetooth = c::AF_BLUETOOTH as isize,
    ///IUCV sockets
    Iucv = c::AF_IUCV as isize,
    /// RxRPC sockets
    RxRpc = c::AF_RXRPC as isize,
    /// mISDN sockets
    Isdn = c::AF_ISDN as isize,
    /// Phonet sockets
    Phonet = c::AF_PHONET as isize,
    /// IEEE802154 sockets
    Ieee802154 = c::AF_IEEE802154 as isize,
    /// CAIF sockets
    Caif = c::AF_CAIF as isize,
    /// Algorithm sockets
    Algorithm = c::AF_ALG as isize,
    /// NFC sockets
    Nfc = c::AF_NFC as isize,
    /// vSockets
    VSocket = c::AF_VSOCK as isize,
    /// Kernel Connection Multiplexor
    Kcm = c::AF_KCM as isize,
    /// Qualcomm IPC Router
    QIpcRtr = c::AF_QIPCRTR as isize,
    /// smc sockets: reserve number for PF_SMC protocol family that reuses AF_INET
    /// address family
    Smc = c::AF_SMC as isize,
}


pub enum ProtocolFamily {
    /// Ensures that zero is not a valid option
    Unspec = c::PF_UNSPEC as isize,
    /// Unix domain socket
    Unix = c::PF_UNIX as isize,
    /// Internet IP Protocol
    Ip = c::PF_INET as isize,
    /// Amateur Radio AX.25
    Ax25 = c::PF_AX25 as isize,
    /// Novell IPX
    Ipx = c::PF_IPX as isize,
    /// AppleTalk DDP 
    AppleTalk = c::PF_APPLETALK as isize,
    /// Amateur Radio NET/ROM 
    NetRom = c::PF_NETROM as isize,
    /// Multiprotocol bridge 
    Bridge = c::PF_BRIDGE as isize,
    /// ATM PVCs
    AtmPvc = c::PF_ATMPVC as isize,
    /// Reserved for X.25 project 
    X25 = c::PF_X25 as isize,
    /// IP version 6
    Ipv6 = c::PF_INET6 as isize,
    /// Amateur Radio X.25 PLP
    Rose = c::PF_ROSE as isize,
    /// Reserved for DECnet project
    DecNet = c::PF_DECnet as isize,
    /// Reserved for 802.2LLC project
    NetBeui = c::PF_NETBEUI as isize,
    /// Security callback pseudo AF
    Security = c::PF_SECURITY as isize,
    /// PF_KEY key management API
    Key = c::PF_KEY as isize,
    Netlink = c::PF_NETLINK as isize,
    /// Packet family
    Packet = c::PF_PACKET as isize,
    /// Ash
    Ash = c::PF_ASH as isize,
    /// Acorn Econet
    Econet = c::PF_ECONET as isize,
    /// ATM SVCs
    AtmSvc = c::PF_ATMSVC as isize,
    /// RDS sockets 
    Rds = c::PF_RDS as isize,
    /// Linux SNA Project (nutters!) */
    Sna = c::PF_SNA as isize,
    /// IRDA sockets
    Irda = c::PF_IRDA as isize,
    /// PPPoX sockets
    PPPox = c::PF_PPPOX as isize,
    /// Wanpipe API Sockets
    Wanpipe = c::PF_WANPIPE as isize,
    /// Linux LLC
    Llc = c::PF_LLC as isize,
    /// Native InfiniBand address
    InfiniBand = c::PF_IB as isize,
    /// MPLS
    Mpls = c::PF_MPLS as isize,
    /// Controller Area Network
    Can = c::PF_CAN as isize,
    /// TIPC sockets
    Tipc = c::PF_TIPC as isize,
    /// Bluetooth sockets
    Bluetooth = c::PF_BLUETOOTH as isize,
    ///IUCV sockets
    Iucv = c::PF_IUCV as isize,
    /// RxRPC sockets
    RxRpc = c::PF_RXRPC as isize,
    /// mISDN sockets
    Isdn = c::PF_ISDN as isize,
    /// Phonet sockets
    Phonet = c::PF_PHONET as isize,
    /// IEEE802154 sockets
    Ieee802154 = c::PF_IEEE802154 as isize,
    /// CAIF sockets
    Caif = c::PF_CAIF as isize,
    /// Algorithm sockets
    Algorithm = c::PF_ALG as isize,
    /// NFC sockets
    Nfc = c::PF_NFC as isize,
    /// vSockets
    VSocket = c::PF_VSOCK as isize,
    /// Kernel Connection Multiplexor
    Kcm = c::PF_KCM as isize,
    /// Qualcomm IPC Router
    QIpcRtr = c::PF_QIPCRTR as isize,
    /// smc sockets: reserve number for PF_SMC protocol family that reuses PF_INET
    /// address family
    Smc = c::PF_SMC as isize,

}

pub fn socket_syscall(
    domain: self::AddressFamily,
    description: net::SocketDescription,
    protocol: ether::Protocol
) -> Result<c_int,io::Error> {

    let mut sock_type_num : c_uint = description.sock_type as u32;
    if description.nonblock {
        sock_type_num += 00004000;
    }
    if description.close_on_exec {
        sock_type_num += 02000000;
    }

    match unsafe { libc_socket(
        domain as i32,
        sock_type_num as i32,
        (protocol as u32).to_be() as i32
    ) } {
        -1 => Err(io::Error::last_os_error()),
        n if n > 0 => Ok(n),
        _ => unreachable!("socket syscall returned 0!")
    }

}
