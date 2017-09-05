
#![allow(non_upper_case_globals)]

use libc::{
    c_uint
};

// Supported address families.

/// Ensures that zero is not a valid option
pub const AF_UNSPEC: c_uint = 0;
/// Unix domain socket
pub const AF_UNIX: c_uint = 1;
/// POSOX name for AF_UNIX
pub const AF_LOCAL: c_uint = 1;
/// Internet IP Protocol
pub const AF_INET: c_uint = 2;
/// Amateur Radio AX.25
pub const AF_AX25: c_uint = 3;
/// Novell IPX
pub const AF_IPX: c_uint = 4;
/// AppleTalk DDP 
pub const AF_APPLETALK: c_uint = 5;
/// Amateur Radio NET/ROM 
pub const AF_NETROM: c_uint = 6;
/// Multiprotocol bridge 
pub const AF_BRIDGE: c_uint = 7;
/// ATM PVCs
pub const AF_ATMPVC: c_uint = 8;
/// Reserved for X.25 project 
pub const AF_X25: c_uint = 9;
/// IP version 6
pub const AF_INET6: c_uint = 10;
/// Amateur Radio X.25 PLP
pub const AF_ROSE: c_uint = 11;
/// Reserved for DECnet project
pub const AF_DECnet: c_uint = 12;
/// Reserved for 802.2LLC project
pub const AF_NETBEUI: c_uint = 13;
/// Security callback pseudo AF
pub const AF_SECURITY: c_uint = 14;
/// PF_KEY key management API
pub const AF_KEY: c_uint = 15;
pub const AF_NETLINK: c_uint = 16;
/// Alias to emulate 4.4BSD
pub const AF_ROUTE: c_uint = AF_NETLINK;
/// Packet family
pub const AF_PACKET: c_uint = 17;
/// Ash
pub const AF_ASH: c_uint = 18;
/// Acorn Econet
pub const AF_ECONET: c_uint = 19;
/// ATM SVCs
pub const AF_ATMSVC: c_uint = 20;
/// RDS sockets 
pub const AF_RDS: c_uint = 21;
/// Linux SNA Project (nutters!) */
pub const AF_SNA: c_uint = 22;
/// IRDA sockets
pub const AF_IRDA: c_uint = 23;
/// PPPoX sockets
pub const AF_PPPOX: c_uint = 24;
/// Wanpipe API Sockets
pub const AF_WANPIPE: c_uint = 25;
/// Linux LLC
pub const AF_LLC: c_uint = 26;
/// Native InfiniBand address
pub const AF_IB: c_uint = 27;
/// MPLS
pub const AF_MPLS: c_uint = 28;
/// Controller Area Network
pub const AF_CAN: c_uint = 29;
/// TIPC sockets
pub const AF_TIPC: c_uint = 30;
/// Bluetooth sockets
pub const AF_BLUETOOTH: c_uint = 31;
///IUCV sockets
pub const AF_IUCV: c_uint = 32;
/// RxRPC sockets
pub const AF_RXRPC: c_uint = 33;
/// mISDN sockets
pub const AF_ISDN: c_uint = 34;
/// Phonet sockets
pub const AF_PHONET: c_uint = 35;
/// IEEE802154 sockets
pub const AF_IEEE802154: c_uint = 36;
/// CAIF sockets
pub const AF_CAIF: c_uint = 37;
/// Algorithm sockets
pub const AF_ALG: c_uint = 38;
/// NFC sockets
pub const AF_NFC: c_uint = 39;
/// vSockets
pub const AF_VSOCK: c_uint = 40;
/// Kernel Connection Multiplexor
pub const AF_KCM: c_uint = 41;
/// Qualcomm IPC Router
pub const AF_QIPCRTR: c_uint = 42;
/// smc sockets: reserve number for PF_SMC protocol family that reuses AF_INET
/// address family
pub const AF_SMC: c_uint = 43;
/// Current max number of AF option values
pub const AF_MAX: c_uint = 44;

/// Protocol families, same as address families.

pub const PF_UNSPEC: c_uint = AF_UNSPEC;
pub const PF_UNIX: c_uint = AF_UNIX;
pub const PF_LOCAL: c_uint = AF_LOCAL;
pub const PF_INET: c_uint = AF_INET;
pub const PF_AX25: c_uint = AF_AX25;
pub const PF_IPX: c_uint = AF_IPX;
pub const PF_APPLETALK: c_uint = AF_APPLETALK;
pub const PF_NETROM: c_uint = AF_NETROM;
pub const PF_BRIDGE: c_uint = AF_BRIDGE;
pub const PF_ATMPVC: c_uint = AF_ATMPVC;
pub const PF_X25: c_uint = AF_X25;
pub const PF_INET6: c_uint = AF_INET6;
pub const PF_ROSE: c_uint = AF_ROSE;
pub const PF_DECnet: c_uint = AF_DECnet;
pub const PF_NETBEUI: c_uint = AF_NETBEUI;
pub const PF_SECURITY: c_uint = AF_SECURITY;
pub const PF_KEY: c_uint = AF_KEY;
pub const PF_NETLINK: c_uint = AF_NETLINK;
pub const PF_ROUTE: c_uint = AF_ROUTE;
pub const PF_PACKET: c_uint = AF_PACKET;
pub const PF_ASH: c_uint = AF_ASH;
pub const PF_ECONET: c_uint = AF_ECONET;
pub const PF_ATMSVC: c_uint = AF_ATMSVC;
pub const PF_RDS: c_uint = AF_RDS;
pub const PF_SNA: c_uint = AF_SNA;
pub const PF_IRDA: c_uint = AF_IRDA;
pub const PF_PPPOX: c_uint = AF_PPPOX;
pub const PF_WANPIPE: c_uint = AF_WANPIPE;
pub const PF_LLC: c_uint = AF_LLC;
pub const PF_IB: c_uint = AF_IB;
pub const PF_MPLS: c_uint = AF_MPLS;
pub const PF_CAN: c_uint = AF_CAN;
pub const PF_TIPC: c_uint = AF_TIPC;
pub const PF_BLUETOOTH: c_uint = AF_BLUETOOTH;
pub const PF_IUCV: c_uint = AF_IUCV;
pub const PF_RXRPC: c_uint = AF_RXRPC;
pub const PF_ISDN: c_uint = AF_ISDN;
pub const PF_PHONET: c_uint = AF_PHONET;
pub const PF_IEEE802154: c_uint = AF_IEEE802154;
pub const PF_CAIF: c_uint = AF_CAIF;
pub const PF_ALG: c_uint = AF_ALG;
pub const PF_NFC: c_uint = AF_NFC;
pub const PF_VSOCK: c_uint = AF_VSOCK;
pub const PF_KCM: c_uint = AF_KCM;
pub const PF_QIPCRTR: c_uint = AF_QIPCRTR;
pub const PF_SMC: c_uint = AF_SMC;
pub const PF_MAX: c_uint = AF_MAX;

/// Maximum queue length specifiable by listen.
pub const SOMAXCONN: c_uint = 128;

/* Flags we can use with send/ and recv. 
   Added those for 1003.1g not all are supported yet
 */
 
pub const MSG_OOB: c_uint = 1;
pub const MSG_PEEK: c_uint = 2;
pub const MSG_DONTROUTE: c_uint = 4;
/// Synonym for MSG_DONTROUTE for DECnet
pub const MSG_TRYHARD: c_uint = MSG_DONTROUTE;
pub const MSG_CTRUNC: c_uint = 8;
/// Do not send. Only probe path f.e. for MTU
pub const MSG_PROBE: c_uint = 0x10;
pub const MSG_TRUNC: c_uint = 0x20;
/// Nonblocking io
pub const MSG_DONTWAIT: c_uint = 0x40;
/// End of record
pub const MSG_EOR: c_uint = 0x80;
/// Wait for a full request
pub const MSG_WAITALL: c_uint = 0x100;
pub const MSG_FIN: c_uint = 0x200;
pub const MSG_SYN: c_uint = 0x400;
/// Confirm path validity
pub const MSG_CONFIRM: c_uint = 0x800;
pub const MSG_RST: c_uint = 0x1000;
/// Fetch message from error queue
pub const MSG_ERRQUEUE: c_uint = 0x2000;
/// Do not generate SIGPIPE
pub const MSG_NOSIGNAL: c_uint = 0x4000;
/// Sender will send more
pub const MSG_MORE: c_uint = 0x8000;
/// recvmmsg(): block until 1+ packets avail
pub const MSG_WAITFORONE: c_uint = 0x10000;
/// sendpage() internal : not the last page
pub const MSG_SENDPAGE_NOTLAST: c_uint = 0x20000;
/// sendmmsg(): more messages coming
pub const MSG_BATCH: c_uint = 0x40000;
pub const MSG_EOF: c_uint = MSG_FIN;

/// Send data in TCP SYN
pub const MSG_FASTOPEN: c_uint = 0x20000000;
/// Set close_on_exec for file descriptor received through SCM_RIGHTS
pub const MSG_CMSG_CLOEXEC: c_uint = 0x40000000;

/*#if defined(CONFIG_COMPAT)
pub const MSG_CMSG_COMPAT 0x80000000 /* This message needs 32 bit fixups */
#else
pub const MSG_CMSG_COMPAT 0 /* We never have 32 bit fixups */
#endif
*/

// Setsockoptions(2) level. Thanks to BSD these must match IPPROTO_xxx

pub const SOL_IP: c_uint = 0;
// pub const SOL_ICMP  1   No-no-no! Due to Linux :-) we cannot use SOL_ICMP=1
pub const SOL_TCP: c_uint = 6;
pub const SOL_UDP: c_uint = 17;
pub const SOL_IPV6: c_uint = 41;
pub const SOL_ICMPV6: c_uint = 58;
pub const SOL_SCTP: c_uint = 132;
/// UDP-Lite (RFC 3828)
pub const SOL_UDPLITE: c_uint = 136;
pub const SOL_RAW: c_uint = 255;
pub const SOL_IPX: c_uint = 256;
pub const SOL_AX25: c_uint = 257;
pub const SOL_ATALK: c_uint = 258;
pub const SOL_NETROM: c_uint = 259;
pub const SOL_ROSE: c_uint = 260;
pub const SOL_DECNET: c_uint = 261;
pub const SOL_X25: c_uint = 262;
pub const SOL_PACKET: c_uint = 263;
/// ATM layer (cell level)
pub const SOL_ATM: c_uint = 264;
/// ATM Adaption Layer (packet level)
pub const SOL_AAL: c_uint = 265;
pub const SOL_IRDA: c_uint = 266;
pub const SOL_NETBEUI: c_uint = 267;
pub const SOL_LLC: c_uint = 268;
pub const SOL_DCCP: c_uint = 269;
pub const SOL_NETLINK: c_uint = 270;
pub const SOL_TIPC: c_uint = 271;
pub const SOL_RXRPC: c_uint = 272;
pub const SOL_PPPOL2TP: c_uint = 273;
pub const SOL_BLUETOOTH: c_uint = 274;
pub const SOL_PNPIPE: c_uint = 275;
pub const SOL_RDS: c_uint = 276;
pub const SOL_IUCV: c_uint = 277;
pub const SOL_CAIF: c_uint = 278;
pub const SOL_ALG: c_uint = 279;
pub const SOL_NFC: c_uint = 280;
pub const SOL_KCM: c_uint = 281;
pub const SOL_TLS: c_uint = 282;

/// IPX options
pub const IPX_TYPE: c_uint = 1;
