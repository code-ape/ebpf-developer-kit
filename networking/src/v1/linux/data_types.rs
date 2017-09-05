
use std::fmt;

use ::v1::linux::ether as ether;

#[derive(Debug)]
#[repr(packed)]
pub struct MacAddress(pub [u8;ether::c::ETH_ALEN as usize]);

impl fmt::Display for MacAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:X}:{:X}:{:X}:{:X}:{:X}:{:X}",
            self.0[0], self.0[1], self.0[2], self.0[3],
            self.0[4], self.0[5]
        )
    }    
}