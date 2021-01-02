use std::net::Ipv4Addr;

use pnet::util::MacAddr;

const OP: usize = 0;
const HTYPE: usize = 1;
const HLEN: usize = 2;
const HOPS: usize = 3;
const XID: usize = 4;
const SECS: usize = 8;
const FLAGS: usize = 10;
const CIADDR: usize = 12;
const YIADDR: usize = 16;
const SIADDR: usize = 20;
const GIADDR: usize = 24;
const CHADDR: usize = 28;
const SNAME: usize = 44;
const FILE: usize = 108;
const OPTIONS: usize = 236;

/**
 * DHCPのパケットを表現する
 */
pub struct DhcpPacket {
    buffer: Vec<u8>,
}

impl DhcpPacket {
    pub fn get_buffer(&self) -> &[u8] {
        self.buffer().as_ref()
    }

    pub fn get_op(&self) -> u8 {
        self.buffer[OP]
    }

    pub fn get_options(&self) -> &[u8] {
        self.buffer[OPTIONS..]
    }

    pub fn get_xid(&self) -> &[u8] {
        self.buffer[XID..SECS]
    }

    pub fn get_flags(&self) -> &[u8] {
        self.buffer[FLAGS..CIADDR]
    }

    pub fn get_giaddr(&self) -> Ipv4Addr {
        let b = &self.buffer[GIADDR..CHADDR];
        Ipv4Addr::new(b[0], b[1], b[2], b[3])
    }

    pub fn get_chaddr(&self) -> MacAddr {
        let b = &self.buffer[CHADDR..SNAME];
        MacAddr::new(b[0], b[1], b[2], b[3], b[4], b[5])
    }

    pub fn get_ciaddr(&self) -> Ipv4Addr {
        let b = &self.buffer[CIADDR..YIADDR];
        Ipv4Addr::new(b[0], b[1], b[2], b[3])
    }
}
