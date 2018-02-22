
use std::mem;

use libc::{
    c_void
};

use ::v1::linux::packet::rust::{
    TPacketBlockDesc,
    TPacketBlockDescV1,
    TPacket3Hdr
};

impl TPacketBlockDesc {
    pub fn into_v1(self) -> Result<TPacketBlockDescV1,TPacketBlockDesc> {
        match self.version {
            1 => Ok( unsafe { mem::transmute(self) } ),
            _ => Err(self)
        }
    }
}

#[derive(Debug)]
pub struct TPacketBlockDescV1Iter<'a> {
    block_desc: &'a mut TPacketBlockDescV1,
    last_index: isize,
    next_offset: u32
}

impl TPacketBlockDescV1 {
    /// Returns iterator `TPacketBlockDescV1Iter` which iterates through the `TPacketBlockDescV1`
    /// providing `&'a TPacket3Hdr`.
    pub fn iter(&mut self) -> TPacketBlockDescV1Iter {
        println!("TPacketBlockDescV1 with {} packets constructing iterator...",
            self.hdr.num_pkts);
        TPacketBlockDescV1Iter {
            block_desc: self,
            last_index: -1,
            next_offset: 0
        }
    }
}

impl<'a> Iterator for TPacketBlockDescV1Iter<'a> {
    type Item = &'a TPacket3Hdr;
    fn next(&mut self) -> Option<&'a TPacket3Hdr> {
        match self.last_index + 1 {
            n if n < (self.block_desc.hdr.num_pkts as isize) => unsafe {
                let block_desc_ptr = self.block_desc as *const TPacketBlockDescV1;
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
