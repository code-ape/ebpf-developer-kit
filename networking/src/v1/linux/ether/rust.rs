
use std::fmt;

use ::v1::linux::data_types::{
    MacAddress
};

use ::v1::linux::ether::c as c;

pub use ::v1::linux::ether::c::{
    // magic constants
    ETH_ALEN,
    ETH_HLEN,
    ETH_ZLEN,
    ETH_DATA_LEN,
    ETH_FRAME_LEN,
    ETH_FCS_LEN,
    ETH_MIN_MTU,
    ETH_MAX_MTU,
    ETH_P_802_3_MIN,
    ethhdr as EthHdr,
};


pub enum Protocol {
    /// Ethernet Loopback packet
    Loop = c::ETH_P_LOOP as isize,
    /// Xerox PUP packet
    Pup = c::ETH_P_PUP as isize,
    /// Xerox PUP Addr Trans packet
    Pupat = c::ETH_P_PUPAT as isize,
    /// TSN (IEEE 1722) packet
    Tsn = c::ETH_P_TSN as isize,
    /// Internet Protocol packet
    Ip = c::ETH_P_IP as isize,
    /// CCITT X.25
    X25 = c::ETH_P_X25 as isize,
    /// Address Resolution packet
    Arp = c::ETH_P_ARP as isize,
    /// G8BPQ AX.25 Ethernet Packet [ NOT AN OFFICIALLY REGISTERED ID ]
    Bpq = c::ETH_P_BPQ as isize,
    /// Xerox IEEE802.3 PUP packet
    IeeePup = c::ETH_P_IEEEPUP as isize,
    /// Xerox IEEE802.3 PUP Addr Trans packet
    IeeePupat = c::ETH_P_IEEEPUPAT as isize,
    /// B.A.T.M.A.N. - Advanced packet [ NOT AN OFFICIALLY REGISTERED ID ]
    Batman = c::ETH_P_BATMAN as isize,
    /// DEC Assigned proto
    Dec = c::ETH_P_DEC as isize,
    /// DEC DNA Dump/Load
    DnaDumpLoad = c::ETH_P_DNA_DL as isize,
    /// DEC DNA Remote Console
    DnaRemoteConsole = c::ETH_P_DNA_RC as isize,
    /// DEC DNA Routing
    DnaRouting = c::ETH_P_DNA_RT as isize,
    /// DEC LAT
    Lat = c::ETH_P_LAT as isize,
    /// DEC Diagnostics
    Diag = c::ETH_P_DIAG as isize,
    /// DEC Customer use
    Cust = c::ETH_P_CUST as isize,
    /// DEC Systems Comms Arch
    Sca = c::ETH_P_SCA as isize,
    /// Trans Ether Bridging
    Teb = c::ETH_P_TEB as isize,
    /// Reverse Addr Res packet
    Rarp = c::ETH_P_RARP as isize,
    /// Appletalk DDP
    Atalk = c::ETH_P_ATALK as isize,
    /// Appletalk AARP
    Aarp = c::ETH_P_AARP as isize,
    /// 802.1Q VLAN Extended Header
    #[allow(non_camel_case_types)]
    _8021Q = c::ETH_P_8021Q as isize,
    /// IPX over DIX
    Ipx = c::ETH_P_IPX as isize,
    /// IPv6 over bluebook
    Ipv6 = c::ETH_P_IPV6 as isize,
    /// IEEE Pause frames. See 802.3 31B
    Pause = c::ETH_P_PAUSE as isize,
    /// Slow Protocol. See 802.3ad 43B
    Slow = c::ETH_P_SLOW as isize,
    /// Web-cache coordination protocol defined in draft-wilson-wrec-wccp-v2-00.txt
    Wccp = c::ETH_P_WCCP as isize,
    /// MPLS Unicast traffic
    MplsUc = c::ETH_P_MPLS_UC as isize,
    /// MPLS Multicast traffic
    MplsMc = c::ETH_P_MPLS_MC as isize,
    /// MultiProtocol Over ATM
    AtmMpoa = c::ETH_P_ATMMPOA as isize,
    /// PPPoE discovery messages
    Disc = c::ETH_P_PPP_DISC as isize,
    /// PPPoE session messages
    PppSes = c::ETH_P_PPP_SES as isize,
    /// HPNA as isize, wlan link local tunnel
    LinkCtl = c::ETH_P_LINK_CTL as isize,
    /// Frame-based ATM Transport over Ethernet
    AtmFate = c::ETH_P_ATMFATE as isize,
    /// Port Access Entity (IEEE 802.1X)
    Pae = c::ETH_P_PAE as isize,
    /// ATA over Ethernet
    Aoe = c::ETH_P_AOE as isize,
    /// 802.1ad Service VLAN
    _8021Ad = c::ETH_P_8021AD as isize,
    /// 802.1 Local Experimental 1. 
    _802x1 = c::ETH_P_802_EX1 as isize,
    /// TIPC
    Tipc = c::ETH_P_TIPC as isize,
    /// 802.1ae MACsec
    MacSec = c::ETH_P_MACSEC as isize,
    /// 802.1ah Backbone Service Tag
    #[allow(non_camel_case_types)]
    _8021Ah = c::ETH_P_8021AH as isize,
    /// 802.1Q MVRP
    Mvrp = c::ETH_P_MVRP as isize,
    /// IEEE 1588 Timesync
    Ieee1588 = c::ETH_P_1588 as isize,
    /// NCSI protocol
    Ncsi = c::ETH_P_NCSI as isize,
    /// IEC 62439-3 PRP/HSRv0
    Prp = c::ETH_P_PRP as isize,
    /// Fibre Channel over Ethernet 
    Fcoe = c::ETH_P_FCOE as isize,
    /// Infiniband over Ethernet
    Iboe = c::ETH_P_IBOE as isize,
    /// TDLS
    Tdls = c::ETH_P_TDLS as isize,
    /// FCoE Initialization Protocol
    Fip = c::ETH_P_FIP as isize,
    /// IEEE 802.21 Media Independent Handover Protocol
    Ieee80221 = c::ETH_P_80221 as isize,
    /// IEC 62439-3 HSRv1
    Hsr = c::ETH_P_HSR as isize,
    /// Ethernet loopback packet as isize, per IEEE 802.3
    Loopback = c::ETH_P_LOOPBACK as isize,
    /// deprecated QinQ VLAN [ NOT AN OFFICIALLY REGISTERED ID ]
    Qinq1 = c::ETH_P_QINQ1 as isize,
    /// deprecated QinQ VLAN [ NOT AN OFFICIALLY REGISTERED ID ]
    Qinq2 = c::ETH_P_QINQ2 as isize,
    /// deprecated QinQ VLAN [ NOT AN OFFICIALLY REGISTERED ID ]
    Qinq3 = c::ETH_P_QINQ3 as isize,
    /// Ethertype DSA [ NOT AN OFFICIALLY REGISTERED ID ]
    Edsa = c::ETH_P_EDSA as isize,
    /// IBM af_iucv [ NOT AN OFFICIALLY REGISTERED ID ]
    Iucv = c::ETH_P_AF_IUCV as isize,


    // Non DIX types. Won't clash for 1500 types.

    /// Dummy type for 802.3 frames
    #[allow(non_camel_case_types)]
    _802_3 = c::ETH_P_802_3 as isize,
    /// Dummy protocol id for AX.25
    Ax25 = c::ETH_P_AX25 as isize,
    /// Every packet (be careful!!!)
    All = c::ETH_P_ALL as isize,
    /// 802.2 frames
    #[allow(non_camel_case_types)]
    _802_2 = c::ETH_P_802_2 as isize,
    /// Internal only
    Snap = c::ETH_P_SNAP as isize,
    /// DEC DDCMP: Internal only
    DDcmp = c::ETH_P_DDCMP as isize,
    /// Dummy type for WAN PPP frames
    WanPpp = c::ETH_P_WAN_PPP as isize,
    /// Dummy type for PPP MP frames
    PppMp = c::ETH_P_PPP_MP as isize,
    /// Localtalk pseudo type
    LocalTalk = c::ETH_P_LOCALTALK as isize,
    /// CAN: Controller Area Network
    Can = c::ETH_P_CAN as isize,
    /// CANFD: CAN flexible data rate
    CanFD = c::ETH_P_CANFD as isize,
    /// Dummy type for Atalk over PPP
    PppTalk = c::ETH_P_PPPTALK as isize,
    /// 802.2 frames
    #[allow(non_camel_case_types)]
    Tr_802_2= c::ETH_P_TR_802_2  as isize,
    /// Mobitex (kaz@cafe.net)
    Mobitex = c::ETH_P_MOBITEX as isize,
    /// Card specific control frames
    Control = c::ETH_P_CONTROL as isize,
    /// Linux-IrDA
    Irda = c::ETH_P_IRDA as isize,
    /// Acorn Econet
    Econet = c::ETH_P_ECONET as isize,
    /// HDLC frames
    Hdlc = c::ETH_P_HDLC as isize,
    /// 1A for ArcNet :-)
    ArcNet = c::ETH_P_ARCNET as isize,
    /// Distributed Switch Arch.
    Dsa = c::ETH_P_DSA as isize,
    /// Trailer switch tagging
    Trailer = c::ETH_P_TRAILER as isize,
    /// Nokia Phonet frames
    Phonet = c::ETH_P_PHONET as isize,
    /// IEEE802.15.4 frame
    #[allow(non_camel_case_types)]
    Ieee802_154= c::ETH_P_IEEE802154 as isize,
    /// ST-Ericsson CAIF protocol
    Caif = c::ETH_P_CAIF as isize,
    /// Multiplexed DSA protocol
    XDsa = c::ETH_P_XDSA as isize,

}
    

impl fmt::Display for EthHdr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Destination: {}, Source: {}, Proto: {:X}",
            MacAddress(self.h_dest),
            MacAddress(self.h_source),
            self.h_proto
        )
    }    
}