
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_macros)]

//! This is a near perfect copy of the `include/uapi/linux/if_packet.h` from
//! the Linux Kernel. It allows Rust code to interlope with the packet system
//! of the Linux Kernel.

use std::fmt;
use std::mem;

use libc::{
    // number types
    __u8,
    __u16,
    __u32,
    __u64,
    c_char,
    c_uchar,
    c_short,
    c_ushort,
    c_int,
    c_uint,
    c_ulong,
    // libc's faulty sockaddr struct
    sockaddr
};

type __aligned_u64 = __u64;


#[allow(deprecated)]
#[deprecated]
#[derive(Debug)]
#[repr(C)]
pub struct sockaddr_pkt {
    pub spkt_family: c_short,
    pub spkt_device: [c_char; 14],
    pub skpt_protocol: __u16 // __be16
}

macro_rules! ImplUnionDebug {
    ($u:ty) => {
        impl fmt::Debug for $u {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(
                    f, "{} {{ union of byte size {} }}",
                    stringify!($u), mem::size_of::<$u>()
                )
            }
        }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct sockaddr_ll {
    pub sll_family: c_short,
    pub sll_protocol: __u16,
    pub sll_ifindex: c_int,
    pub sll_hatype: c_short,
    pub sll_pkttype: c_char,
    pub sll_halen: c_char,
    pub sll_addr: [c_char;8]
}


impl Into<sockaddr> for sockaddr_ll {
    fn into(mut self) -> sockaddr {
        self.sll_family = self.sll_family;
        self.sll_protocol = self.sll_protocol.to_be();
        self.sll_ifindex = self.sll_ifindex;//.to_be();
        unsafe { 
            let p = &self as *const sockaddr_ll;
            *(mem::transmute::<*const sockaddr_ll, *const sockaddr>(p))
         }
    }
}

/// Packet types
/// Copied from linux kernel `include/uapi/linux/if_packet.h`

pub const PACKET_HOST: c_uint = 0;       /* To us */
pub const PACKET_BROADCAST: c_uint = 1;  /* To all */
pub const PACKET_MULTICAST: c_uint = 2;  /* To group */
pub const PACKET_OTHERHOST: c_uint = 3;  /* To someone else  */
pub const PACKET_OUTGOING: c_uint = 4;   /* Outgoing of any type */
pub const PACKET_LOOPBACK: c_uint = 5;   /* MC/BRD frame looped back */
pub const PACKET_USER: c_uint = 6;       /* To user space */
pub const PACKET_KERNEL: c_uint = 7;     /* To kernel space */
/* Unused, PACKET_FASTROUTE and PACKET_LOOPBACK are invisible to user space */
pub const PACKET_FASTROUTE: c_uint = 6; /* Fastrouted frame */



/* Packet socket options */

pub const PACKET_ADD_MEMBERSHIP: c_uint = 1;
pub const PACKET_DROP_MEMBERSHIP: c_uint = 2;
pub const PACKET_RECV_OUTPUT: c_uint = 3;
/* Value 4 is still used by obsolete turbo-packet. */
pub const PACKET_RX_RING: c_uint = 5;
pub const PACKET_STATISTICS: c_uint = 6;
pub const PACKET_COPY_THRESH: c_uint = 7;
pub const PACKET_AUXDATA: c_uint = 8;
pub const PACKET_ORIGDEV: c_uint = 9;
pub const PACKET_VERSION: c_uint = 10;
pub const PACKET_HDRLEN: c_uint = 11;
pub const PACKET_RESERVE: c_uint = 12;
pub const PACKET_TX_RING: c_uint = 13;
pub const PACKET_LOSS: c_uint = 14;
pub const PACKET_VNET_HDR: c_uint = 15;
pub const PACKET_TX_TIMESTAMP: c_uint = 16;
pub const PACKET_TIMESTAMP: c_uint = 17;
pub const PACKET_FANOUT: c_uint = 18;
pub const PACKET_TX_HAS_OFF: c_uint = 19;
pub const PACKET_QDISC_BYPASS: c_uint = 20;
pub const PACKET_ROLLOVER_STATS: c_uint = 21;
pub const PACKET_FANOUT_DATA: c_uint = 22;

pub const PACKET_FANOUT_HASH: c_uint = 0;
pub const PACKET_FANOUT_LB: c_uint = 1;
pub const PACKET_FANOUT_CPU: c_uint = 2;
pub const PACKET_FANOUT_ROLLOVER: c_uint = 3;
pub const PACKET_FANOUT_RND: c_uint = 4;
pub const PACKET_FANOUT_QM: c_uint = 5;
pub const PACKET_FANOUT_CBPF: c_uint = 6;
pub const PACKET_FANOUT_EBPF: c_uint = 7;

pub const PACKET_FANOUT_FLAG_ROLLOVER: c_uint = 0x1000;
pub const PACKET_FANOUT_FLAG_UNIQUEID: c_uint = 0x2000;
pub const PACKET_FANOUT_FLAG_DEFRAG: c_uint = 0x8000;


// TODO: remove when unions without Copy is stable
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct tpacket_stats {
    pub tp_packets: c_uint,
    pub tp_drops: c_uint
}

// TODO: remove when unions without Copy is stable
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct tpacket_stats_v3 {
    pub tp_packets: c_uint,
    pub tp_drops: c_uint,
    pub tp_freeze_q_cnt: c_uint
}

#[derive(Debug)]
#[repr(C)]
pub struct tpacket_rollover_stats {
    pub tp_all: __aligned_u64,
    pub tp_huge:  __aligned_u64,
    pub tp_failed:  __aligned_u64
}

#[repr(C)]
pub union tpacket_stats_u {
    pub stats1: tpacket_stats,
    pub stats3: tpacket_stats_v3
}

ImplUnionDebug!(tpacket_stats_u);

#[derive(Debug)]
#[repr(C)]
pub struct tpacket_auxdata {
    pub tp_status: __u32,
    pub tp_len: __u32,
    pub tp_snaplen: __u32,
    pub tp_mac: __u16,
    pub tp_net: __u16,
    pub tp_vlan_tci: __u16,
    pub tp_vlan_tpid: __u16,
}


// Rx ring - header status
pub const TP_STATUS_KERNEL: c_uint = 0;
pub const TP_STATUS_USER: c_uint = (1 << 0);
pub const TP_STATUS_COPY: c_uint = (1 << 1);
pub const TP_STATUS_LOSING: c_uint = (1 << 2);
pub const TP_STATUS_CSUMNOTREADY: c_uint = (1 << 3);
pub const TP_STATUS_VLAN_VALID: c_uint = (1 << 4); /* auxdata has valid tp_vlan_tci */
pub const TP_STATUS_BLK_TMO: c_uint = (1 << 5);
pub const TP_STATUS_VLAN_TPID_VALID: c_uint = (1 << 6); /* auxdata has valid tp_vlan_tpid */
pub const TP_STATUS_CSUM_VALID: c_uint = (1 << 7);

// Tx ring - header status
pub const TP_STATUS_AVAILABLE: c_uint = 0;
pub const TP_STATUS_SEND_REQUEST: c_uint = (1 << 0);
pub const TP_STATUS_SENDING: c_uint = (1 << 1);
pub const TP_STATUS_WRONG_FORMAT: c_uint = (1 << 2);

// Rx and Tx ring - header status
pub const TP_STATUS_TS_SOFTWARE: c_uint = (1 << 29);
pub const TP_STATUS_TS_SYS_HARDWARE: c_uint = (1 << 30); /* deprecated, never set */
pub const TP_STATUS_TS_RAW_HARDWARE: c_uint = (1 << 31);

// Rx ring - feature request bits
pub const TP_FT_REQ_FILL_RXHASH: c_uint = 0x1;

#[derive(Debug)]
#[repr(C)]
pub struct tpacket_hdr {
    pub tp_status: c_ulong,
    pub tp_len: c_uint,
    pub tp_snaplen: c_uint,
    pub tp_mac: c_ushort,
    pub tp_net: c_ushort,
    pub tp_sec: c_uint,
    pub tp_usec: c_uint
}

pub const TPACKET_ALIGNMENT: c_uint = 16;

#[macro_export]
macro_rules! TPACKET_ALIGN {
    ($x:expr) => ((x+TPACKET_ALIGNMENT-1)&~(TPACKET_ALIGNMENT-1))
}

#[macro_export]
macro_rules! TPACKET_HDRLEN {
    () => (
        TPACKET_ALIGN!(mem::size_of::<tpacket_hdr>())
            + mem::size_of::<sockaddr_ll>()
    )
}


#[derive(Debug)]
#[repr(C)]
pub struct tpacket2_hdr {
    pub tp_status: __u32,
    pub tp_len: __u32,
    pub tp_snaplen: __u32,
    pub tp_mac: __u16,
    pub tp_net: __u16,
    pub tp_sec: __u32,
    pub tp_nsec: __u32,
    pub tp_vlan_tci: __u16,
    pub tp_vlan_tpid: __u32,
    pub tp_padding: [__u8;4]
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct tpacket_hdr_variant1 {
    pub tp_rxhash: __u32,
    pub tp_vlan_tci: __u32,
    pub tp_vlan_tpid: __u16,
    pub tp_padding: __u16
}

#[derive(Debug)]
#[repr(C)]
pub struct tpacket3_hdr {
    pub tp_next_offset: __u32,
    pub tp_sec: __u32,
    pub tp_nsec: __u32,
    pub tp_snaplen: __u32,
    pub tp_len: __u32,
    pub tp_status: __u32,
    pub tp_mac: __u16,
    pub tp_net: __u16,
    /* pkt_hdr variants */
    pub hv1: tpacket_hdr_variant1,
    pub tp_padding: [__u8;8]
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct tpacket_bd_ts {
    pub ts_sec: c_uint,
    pub ts_usec_or_nsec: c_uint
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct tpacket_hdr_v1 {
    pub block_status: __u32,
    pub num_pkts: __u32,
    pub offset_to_first_pkt: __u32,
    // number of valid bytes (including padding)
    // blk_len <= tp_block_size
    pub blk_len: __u32,
    /*
     * Quite a few uses of sequence number:
     * 1. Make sure cache flush etc worked.
     *    Well, one can argue - why not use the increasing ts below?
     *    But look at 2. below first.
     * 2. When you pass around blocks to other user space decoders,
     *    you can see which blk[s] is[are] outstanding etc.
     * 3. Validate kernel code.
     */
    pub seq_num: __aligned_u64,

    /*
     * ts_last_pkt:
     *
     * Case 1.  Block has 'N'(N >=1) packets and TMO'd(timed out)
     *          ts_last_pkt == 'time-stamp of last packet' and NOT the
     *          time when the timer fired and the block was closed.
     *          By providing the ts of the last packet we can absolutely
     *          guarantee that time-stamp wise, the first packet in the
     *          next block will never precede the last packet of the
     *          previous block.
     * Case 2.  Block has zero packets and TMO'd
     *          ts_last_pkt = time when the timer fired and the block
     *          was closed.
     * Case 3.  Block has 'N' packets and NO TMO.
     *          ts_last_pkt = time-stamp of the last pkt in the block.
     *
     * ts_first_pkt:
     *          Is always the time-stamp when the block was opened.
     *          Case a) ZERO packets
     *              No packets to deal with but atleast you know the
     *              time-interval of this block.
     *          Case b) Non-zero packets
     *              Use the ts of the first packet in the block.
     *
     */
    pub tpacket_bd_ts: tpacket_bd_ts
}

#[repr(C)]
pub union tpacket_bd_header_u {
    pub hb1: tpacket_hdr_v1
}
ImplUnionDebug!(tpacket_bd_header_u);


#[derive(Debug)]
#[repr(C)]
pub struct tpacket_block_desc {
    pub version: __u32,
    pub offset_to_priv: __u32,
    pub hdr: tpacket_bd_header_u
}

#[macro_export]
macro_rules! TPACKET2_HDRLEN {
    () => (
        TPACKET_ALIGN!(mem::size_of::<tpacket2_hdr>())
            + mem::size_of::<sockaddr_ll>()
    )}

#[macro_export]
macro_rules! TPACKET3_HDRLEN {
    () => (
        TPACKET_ALIGN!(mem::size_of::<tpacket3_hdr>())
            + mem::size_of::<sockaddr_ll>()
    )
}



#[derive(Debug,PartialEq)]
#[repr(C)]
pub enum tpacket_versions {
    TPACKET_V1,
    TPACKET_V2,
    TPACKET_V3
}

/*
   Frame structure:

   - Start. Frame must be aligned to TPACKET_ALIGNMENT=16
   - struct tpacket_hdr
   - pad to TPACKET_ALIGNMENT=16
   - struct sockaddr_ll
   - Gap, chosen so that packet data (Start+tp_net) alignes to TPACKET_ALIGNMENT=16
   - Start+tp_mac: [ Optional MAC header ]
   - Start+tp_net: Packet data, aligned to TPACKET_ALIGNMENT=16.
   - Pad to align to TPACKET_ALIGNMENT=16
 */

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct tpacket_req {
    pub tp_block_size: c_uint,  /* Minimal size of contiguous block */
    pub tp_block_nr: c_uint,    /* Number of blocks */
    pub tp_frame_size: c_uint,  /* Size of frame */
    pub tp_frame_nr: c_uint,    /* Total number of frames */
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct tpacket_req3 {
    pub tp_block_size: c_uint,  /* Minimal size of contiguous block */
    pub tp_block_nr: c_uint,    /* Number of blocks */
    pub tp_frame_size: c_uint,  /* Size of frame */
    pub tp_frame_nr: c_uint,    /* Total number of frames */
    pub tp_retire_blk_tov: c_uint, /* timeout in msecs */
    pub tp_sizeof_priv: c_uint, /* offset to private data area */
    pub tp_feature_req_word: c_uint
}

#[repr(C)]
pub union tpacket_req_u {
    req: tpacket_req,
    req3: tpacket_req3
}

ImplUnionDebug!(tpacket_req_u);

#[derive(Debug)]
#[repr(C)]
pub struct packet_mreq {
    pub mr_ifindex: c_int,
    pub mr_type: c_ushort,
    pub mr_alen: c_ushort,
    pub mr_address: [c_uchar;8]
}

pub const PACKET_MR_MULTICAST: c_uint = 0;
pub const PACKET_MR_PROMISC: c_uint = 1;
pub const PACKET_MR_ALLMULTI: c_uint = 2;
pub const PACKET_MR_UNICAST: c_uint = 3;

