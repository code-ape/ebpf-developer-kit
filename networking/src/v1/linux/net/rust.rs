
use std::mem;

use ::v1::linux::net::c as c;

pub use ::v1::linux::net::c::{
    sock_type,
    SOCK_CLOEXEC as SockCloExec,
    SOCK_NONBLOCK as SockNonBlock
};

#[repr(C)]
#[derive(Debug)]
pub enum SockType {
    /// stream (connection) socket
    Stream = 1,
    /// datagram (connectionless) socket
    Datagram = 2,
    /// raw socket
    Raw = 3,
    /// reliably delivered message
    Rdm = 4,
    /// sequential packet socket
    SequentialPacket = 5,
    /// datagram congestion control protocol socket
    Dccp = 6,
    /// Linux specific way of getting packets at the dev level. For writing
    /// rarp and other similar things on the user level.
    Packet = 10
}

impl Into<c::sock_type> for SockType {
    fn into(self) -> c::sock_type {
        unsafe { mem::transmute(self) }
    }
}
