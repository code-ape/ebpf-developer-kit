
#![allow(non_camel_case_types)]

#[repr(C)]
#[derive(Debug)]
pub enum sock_type {
    /// stream (connection) socket
    SOCK_STREAM = 1,
    /// datagram (connectionless) socket
    SOCK_DGRAM = 2,
    /// raw socket
    SOCK_RAW = 3,
    /// reliably delivered message
    SOCK_RDM = 4,
    /// sequential packet socket
    SOCK_SEQPACKET = 5,
    /// datagram congestion control protocol socket
    SOCK_DCCP = 6,
    /// Linux specific way of getting packets at the dev level. For writing
    /// rarp and other similar things on the user level.
    SOCK_PACKET = 10
} 