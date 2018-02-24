
use ::v1::linux::net::rust::*;

pub struct SocketDescription {
    pub sock_type: SockType,
    pub nonblock: bool,
    pub close_on_exec: bool
}