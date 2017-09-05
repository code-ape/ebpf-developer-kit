
use std::mem;

use ::v1::linux::net::c as c;

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

pub struct SocketDescription {
    pub sock_type: SockType,
    pub nonblock: bool,
    pub close_on_exec: bool
}

impl Into<c::sock_type> for SockType {
    fn into(self) -> c::sock_type {
        unsafe { mem::transmute(self) }
    }
}
