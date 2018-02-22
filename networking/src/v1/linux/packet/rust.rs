
use std::fmt;

use ::v1::linux::packet::c as c;
use ::v1::linux::ether::{
    EthHdr
};

use libc::__u32;

pub use ::v1::linux::packet::c::{
    sockaddr_ll as SockAddrLL,
    tpacket_stats as TPacketStats,
    tpacket_stats_v3 as TpacketStatsV3,
    tpacket_rollover_stats as TPacketRolloverStats,
    tpacket_stats_u as TPacketStatsU,
    tpacket_auxdata as TPacketAuxData,
    tpacket_hdr as TPacketHdr,
    tpacket2_hdr as TPacket2Hdr,
    tpacket3_hdr as TPacket3Hdr,
    tpacket_hdr_variant1 as TPacketHdrVariant1,
    tpacket_bd_ts as TPacketBdTs,
    tpacket_hdr_v1 as TPacketHdrV1,
    tpacket_bd_header_u as TPacketBdHeaderU,
    // Implement TPacketBlockDesc enum over for safety
    tpacket_block_desc as TPacketBlockDesc,
    tpacket_versions as TPacketVersions,
    tpacket_req as TPacketReq,
    tpacket_req3 as TPacketReq3,
    tpacket_req_u as TPacketReqU,
    packet_mreq as PacketMReq,
    // Rx ring - header status
    TP_STATUS_KERNEL,
    TP_STATUS_USER,
    TP_STATUS_COPY,
    TP_STATUS_LOSING,
    TP_STATUS_CSUMNOTREADY,
    TP_STATUS_VLAN_VALID,
    TP_STATUS_BLK_TMO,
    TP_STATUS_VLAN_TPID_VALID,
    TP_STATUS_CSUM_VALID,
    // Tx ring - header status
    TP_STATUS_AVAILABLE,
    TP_STATUS_SEND_REQUEST,
    TP_STATUS_SENDING,
    TP_STATUS_WRONG_FORMAT,
    // Rx and Tx ring - header status
    TP_STATUS_TS_SOFTWARE,
    TP_STATUS_TS_SYS_HARDWARE,
    TP_STATUS_TS_RAW_HARDWARE,
    // Rx ring - feature request bits
    TP_FT_REQ_FILL_RXHASH
};

pub enum PacketType {
    PacketHost = c::PACKET_HOST as isize,
    PacketBroadcast = c::PACKET_BROADCAST as isize,
    PacketMulticast = c::PACKET_MULTICAST as isize,
    PacketOtherhost = c::PACKET_OTHERHOST as isize,
    PacketOutgoing = c::PACKET_OUTGOING as isize,
    PacketLoopback = c::PACKET_LOOPBACK as isize,
    PacketUser = c::PACKET_USER as isize,
    PacketKernel = c::PACKET_KERNEL as isize,
    //PacketFastroute = c::PACKET_FASTROUTE as isize,
}

pub enum PacketOption {
    AddMembership = c::PACKET_ADD_MEMBERSHIP as isize,
    DropMembership = c::PACKET_DROP_MEMBERSHIP as isize,
    RecvOutput = c::PACKET_RECV_OUTPUT as isize,
    // Value 4 is still used by obsolete turbo-packet.
    RxRing = c::PACKET_RX_RING as isize,
    Statistics = c::PACKET_STATISTICS as isize,
    CopyThresh = c::PACKET_COPY_THRESH as isize,
    AuxData = c::PACKET_AUXDATA as isize,
    OrigDev = c::PACKET_ORIGDEV as isize,
    Version = c::PACKET_VERSION as isize,
    HdrLen = c::PACKET_HDRLEN as isize,
    Reserve = c::PACKET_RESERVE as isize,
    TxRing = c::PACKET_TX_RING as isize,
    Loss = c::PACKET_LOSS as isize,
    VnetHdr = c::PACKET_VNET_HDR as isize,
    TxTimestamp = c::PACKET_TX_TIMESTAMP as isize,
    Timestamp = c::PACKET_TIMESTAMP as isize,
    Fanout = c::PACKET_FANOUT as isize,
    TxHasOff = c::PACKET_TX_HAS_OFF as isize,
    QdiscBypass = c::PACKET_QDISC_BYPASS as isize,
    RolloverStats = c::PACKET_ROLLOVER_STATS as isize,
    FanoutData = c::PACKET_FANOUT_DATA as isize,
}

pub enum FanoutOption {
    Hash = c::PACKET_FANOUT_HASH as isize,
    LB = c::PACKET_FANOUT_LB as isize,
    CPU = c::PACKET_FANOUT_CPU as isize,
    Rollover = c::PACKET_FANOUT_ROLLOVER as isize,
    RND = c::PACKET_FANOUT_RND as isize,
    QM = c::PACKET_FANOUT_QM as isize,
    Cbpf = c::PACKET_FANOUT_CBPF as isize,
    Ebpf = c::PACKET_FANOUT_EBPF as isize
}


pub enum PacketMr {
    Multicast = c::PACKET_MR_MULTICAST as isize,
    Promisc = c::PACKET_MR_PROMISC as isize,
    AllMulti = c::PACKET_MR_ALLMULTI as isize,
    Unicast = c::PACKET_MR_UNICAST as isize,
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
pub struct TPacketBlockDescV1 {
    pub version: __u32,
    pub offset_to_priv: __u32,
    pub hdr: TPacketHdrV1
}
