
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_macros)]

//! Rust copy of `include/uapi/linux/if_ether.h` from Linux Kernel
//!  
//! INET An implementation of the TCP/IP protocol suite for the LINUX
//! operating system.  INET is implemented using the  BSD Socket
//! interface as the means of communication with the user level.
//!
//! Global definitions for the Ethernet IEEE 802.3 interface.
//!
//! Version: @(#)if_ether.h 1.0.1a 02/08/94
//!
//! Author: Fred N. van Kempen, <waltje@uWalt.NL.Mugnet.ORG>
//!         Donald Becker, <becker@super.org>
//!         Alan Cox, <alan@lxorguk.ukuu.org.uk>
//!         Steve Whitehouse, <gw7rrm@eeshack3.swan.ac.uk>
//!
//! This program is free software; you can redistribute it and/or
//! modify it under the terms of the GNU General Public License
//! as published by the Free Software Foundation; either version
//! 2 of the License, or (at your option) any later version.


use libc::{
    __u16,
    c_uint,
    c_uchar
};


// IEEE 802.3 Ethernet magic constants. The frame sizes omit the preamble
// and FCS/CRC (frame check sequence).

/// Octets in one ethernet addr
pub const ETH_ALEN: c_uint = 6;
/// Total octets in header
pub const ETH_HLEN: c_uint = 14;
/// Min. octets in frame sans FCS
pub const ETH_ZLEN: c_uint = 60;
/// Max. octets in payload
pub const ETH_DATA_LEN: c_uint = 1500;
/// Max. octets in frame sans FCS
pub const ETH_FRAME_LEN: c_uint = 1514;
/// Octets in the FCS
pub const ETH_FCS_LEN: c_uint = 4;

/// Min IPv4 MTU per RFC791
pub const ETH_MIN_MTU: c_uint = 68;
/// 65535, same as IP_MAX_MTU
pub const ETH_MAX_MTU: c_uint = 0xFFFF;

/// If the value in the ethernet type is less than this value then the frame is
/// Ethernet II. Else it is 802.3
pub const ETH_P_802_3_MIN: c_uint = 0x0600;


// These are the defined Ethernet Protocol ID's.

/// Ethernet Loopback packet
pub const ETH_P_LOOP: c_uint = 0x0060;
/// Xerox PUP packet
pub const ETH_P_PUP: c_uint = 0x0200;
/// Xerox PUP Addr Trans packet
pub const ETH_P_PUPAT: c_uint = 0x0201;
/// TSN (IEEE 1722) packet
pub const ETH_P_TSN: c_uint = 0x22F0;
/// Internet Protocol packet
pub const ETH_P_IP: c_uint = 0x0800;
/// CCITT X.25
pub const ETH_P_X25: c_uint = 0x0805;
/// Address Resolution packet
pub const ETH_P_ARP: c_uint = 0x0806;
/// G8BPQ AX.25 Ethernet Packet [ NOT AN OFFICIALLY REGISTERED ID ]
pub const ETH_P_BPQ: c_uint = 0x08FF;
/// Xerox IEEE802.3 PUP packet
pub const ETH_P_IEEEPUP: c_uint = 0x0a00; 
/// Xerox IEEE802.3 PUP Addr Trans packet
pub const ETH_P_IEEEPUPAT: c_uint = 0x0a01;
/// B.A.T.M.A.N.-Advanced packet [ NOT AN OFFICIALLY REGISTERED ID ]
pub const ETH_P_BATMAN: c_uint = 0x4305;
/// DEC Assigned proto
pub const ETH_P_DEC: c_uint = 0x6000; 
/// DEC DNA Dump/Load
pub const ETH_P_DNA_DL: c_uint = 0x6001;
/// DEC DNA Remote Console
pub const ETH_P_DNA_RC: c_uint = 0x6002;
/// DEC DNA Routing
pub const ETH_P_DNA_RT: c_uint = 0x6003;
/// DEC LAT
pub const ETH_P_LAT: c_uint = 0x6004;
/// DEC Diagnostics
pub const ETH_P_DIAG: c_uint = 0x6005;
/// DEC Customer use
pub const ETH_P_CUST: c_uint = 0x6006;
/// DEC Systems Comms Arch
pub const ETH_P_SCA: c_uint = 0x6007;
/// Trans Ether Bridging
pub const ETH_P_TEB: c_uint = 0x6558;
/// Reverse Addr Res packet
pub const ETH_P_RARP: c_uint = 0x8035;
/// Appletalk DDP
pub const ETH_P_ATALK: c_uint = 0x809B;
/// Appletalk AARP
pub const ETH_P_AARP: c_uint = 0x80F3;
/// 802.1Q VLAN Extended Header 
pub const ETH_P_8021Q: c_uint = 0x8100;
/// IPX over DIX
pub const ETH_P_IPX: c_uint = 0x8137;
/// IPv6 over bluebook
pub const ETH_P_IPV6: c_uint = 0x86DD;
/// IEEE Pause frames. See 802.3 31B
pub const ETH_P_PAUSE: c_uint = 0x8808;
/// Slow Protocol. See 802.3ad 43B
pub const ETH_P_SLOW: c_uint = 0x8809;
/// Web-cache coordination protocol defined in draft-wilson-wrec-wccp-v2-00.txt
pub const ETH_P_WCCP: c_uint = 0x883E;
/// MPLS Unicast traffic
pub const ETH_P_MPLS_UC: c_uint = 0x8847;
/// MPLS Multicast traffic
pub const ETH_P_MPLS_MC: c_uint = 0x8848;
/// MultiProtocol Over ATM
pub const ETH_P_ATMMPOA: c_uint = 0x884c;
/// PPPoE discovery messages
pub const ETH_P_PPP_DISC: c_uint = 0x8863;
/// PPPoE session messages
pub const ETH_P_PPP_SES: c_uint = 0x8864;
/// HPNA, wlan link local tunnel
pub const ETH_P_LINK_CTL: c_uint = 0x886c;
/// Frame-based ATM Transport over Ethernet
pub const ETH_P_ATMFATE: c_uint = 0x8884;
/// Port Access Entity (IEEE 802.1X)
pub const ETH_P_PAE: c_uint = 0x888E;
/// ATA over Ethernet
pub const ETH_P_AOE: c_uint = 0x88A2;
/// 802.1ad Service VLAN
pub const ETH_P_8021AD: c_uint = 0x88A8;
/// 802.1 Local Experimental 1. 
pub const ETH_P_802_EX1: c_uint = 0x88B5;
/// TIPC
pub const ETH_P_TIPC: c_uint = 0x88CA;
/// 802.1ae MACsec
pub const ETH_P_MACSEC: c_uint = 0x88E5;
/// 802.1ah Backbone Service Tag
pub const ETH_P_8021AH: c_uint = 0x88E7;
/// 802.1Q MVRP
pub const ETH_P_MVRP: c_uint = 0x88F5;
/// IEEE 1588 Timesync
pub const ETH_P_1588: c_uint = 0x88F7;
/// NCSI protocol
pub const ETH_P_NCSI: c_uint = 0x88F8;
/// IEC 62439-3 PRP/HSRv0
pub const ETH_P_PRP: c_uint = 0x88FB;
/// Fibre Channel over Ethernet 
pub const ETH_P_FCOE: c_uint = 0x8906;
/// Infiniband over Ethernet
pub const ETH_P_IBOE: c_uint = 0x8915;
/// TDLS
pub const ETH_P_TDLS: c_uint = 0x890D;
/// FCoE Initialization Protocol
pub const ETH_P_FIP: c_uint = 0x8914;
/// IEEE 802.21 Media Independent Handover Protocol
pub const ETH_P_80221: c_uint = 0x8917;
/// IEC 62439-3 HSRv1
pub const ETH_P_HSR: c_uint = 0x892F;
/// Ethernet loopback packet, per IEEE 802.3
pub const ETH_P_LOOPBACK: c_uint = 0x9000;
/// deprecated QinQ VLAN [ NOT AN OFFICIALLY REGISTERED ID ]
pub const ETH_P_QINQ1: c_uint = 0x9100;
/// deprecated QinQ VLAN [ NOT AN OFFICIALLY REGISTERED ID ]
pub const ETH_P_QINQ2: c_uint = 0x9200;
/// deprecated QinQ VLAN [ NOT AN OFFICIALLY REGISTERED ID ]
pub const ETH_P_QINQ3: c_uint = 0x9300;
/// Ethertype DSA [ NOT AN OFFICIALLY REGISTERED ID ]
pub const ETH_P_EDSA: c_uint = 0xDADA;
/// IBM af_iucv [ NOT AN OFFICIALLY REGISTERED ID ]
pub const ETH_P_AF_IUCV: c_uint = 0xFBFB;


// Non DIX types. Won't clash for 1500 types.

/// Dummy type for 802.3 frames
pub const ETH_P_802_3: c_uint = 0x0001;
/// Dummy protocol id for AX.25
pub const ETH_P_AX25: c_uint = 0x0002;
/// Every packet (be careful!!!)
pub const ETH_P_ALL: c_uint = 0x0003;
/// 802.2 frames
pub const ETH_P_802_2: c_uint = 0x0004;
/// Internal only
pub const ETH_P_SNAP: c_uint = 0x0005;
/// DEC DDCMP: Internal only
pub const ETH_P_DDCMP: c_uint = 0x0006;
/// Dummy type for WAN PPP frames
pub const ETH_P_WAN_PPP: c_uint = 0x0007;
/// Dummy type for PPP MP frames
pub const ETH_P_PPP_MP: c_uint = 0x0008;
/// Localtalk pseudo type
pub const ETH_P_LOCALTALK: c_uint = 0x0009;
/// CAN: Controller Area Network
pub const ETH_P_CAN: c_uint = 0x000C;
/// CANFD: CAN flexible data rate
pub const ETH_P_CANFD: c_uint = 0x000D;
/// Dummy type for Atalk over PPP
pub const ETH_P_PPPTALK: c_uint = 0x0010;
/// 802.2 frames
pub const ETH_P_TR_802_2 : c_uint = 0x0011;
/// Mobitex (kaz@cafe.net)
pub const ETH_P_MOBITEX: c_uint = 0x0015;
/// Card specific control frames
pub const ETH_P_CONTROL: c_uint = 0x0016;
/// Linux-IrDA
pub const ETH_P_IRDA: c_uint = 0x0017;
/// Acorn Econet
pub const ETH_P_ECONET: c_uint = 0x0018;
/// HDLC frames
pub const ETH_P_HDLC: c_uint = 0x0019;
/// 1A for ArcNet :-)
pub const ETH_P_ARCNET: c_uint = 0x001A;
/// Distributed Switch Arch.
pub const ETH_P_DSA: c_uint = 0x001B;
/// Trailer switch tagging
pub const ETH_P_TRAILER: c_uint = 0x001C;
/// Nokia Phonet frames
pub const ETH_P_PHONET: c_uint = 0x00F5;
/// IEEE802.15.4 frame
pub const ETH_P_IEEE802154: c_uint = 0x00F6;
/// ST-Ericsson CAIF protocol
pub const ETH_P_CAIF: c_uint = 0x00F7;
/// Multiplexed DSA protocol
pub const ETH_P_XDSA: c_uint = 0x00F8;


/// Ethernet frame header
#[derive(Debug,Copy,Clone)]
#[repr(packed)]
pub struct ethhdr {
    pub h_dest: [c_uchar;ETH_ALEN as usize],
    pub h_source: [c_uchar;ETH_ALEN as usize],
    pub h_proto: __u16 //__be16
}
