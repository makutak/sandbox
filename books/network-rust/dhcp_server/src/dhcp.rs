use std::net::Ipv4Addr;

use pnet::{packet::PrimitiveValues, util::MacAddr};

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

    pub fn set_op(&mut self, op: u8) {
        self.buffer[OP] = op;
    }

    pub fn set_htype(&mut self, htype: u8) {
        self.buffer[HTYPE] = htype;
    }

    pub fn set_hlen(&mut self, hlen: u8) {
        self.buffer[HLEN] = hlen;
    }

    pub fn set_xid(&mut self, xid: &[u8]) {
        self.buffer[XID..SECS].copy_from_slice(xid);
    }

    pub fn set_flags(&mut self, flags: &[u8]) {
        self.buffer[FLAGS..CIADDR].copy_from_slice(flags);
    }

    pub fn set_ciaddr(&mut self, ciaddr: Ipv4Addr) {
        self.buffer[CIADDR..YIADDR].copy_from_slice(&ciaddr.octets());
    }

    pub fn set_yiaddr(&mut self, yiaddr: Ipv4Addr) {
        self.buffer[YIADDR..SIADDR].copy_from_slice(&yiaddr.octets());
    }

    pub fn set_giaddr(&mut self, giaddr: Ipv4Addr) {
        self.buffer[GIADDR..CHADDR].copy_from_slice(&giaddr.octets());
    }

    pub fn set_chaddr(&mut self, chaddr: MacAddr) {
        let t = chaddr.to_primitive_values();
        let macaddr_value = [t.0, t.1, t.2, t.3, t.4, t.5];
        self.buffer[CHADDR..CHADDR + 6].copy_from_slice(&macaddr_value);
    }

    pub fn set_option(
        &mut self,
        cursor: &mut usize,
        code: u8,
        len: usize,
        contents: Option<&[u8]>,
    ) {
        self.buffer[*cursor] = code;
        if code == OPTION_END {
            // 値は存在しないため
            return;
        }

        *cursor += 1;
        self.buffer[*cursor] = len as u8;
        *cursor += 1;
        if let Some(contents) = contents {
            self.buffer[*cursor..*cursor + contents.len()].copy_from_slice(contents);
        }
        *cursor += len;
    }

    pub fn set_magic_cookie(&mut self, cusor: &mut usize) {
        self.buffer[*cusor..*cusor + 4].copy_from_slice(&[0x63, 0x82, 0x52, 0x63]);
        *cusor += 4;
    }

    pub fn get_option(&self, option_code: u8) -> Option<Vec<u8>> {
        // 最初の4バイトとはクッキーなので飛ばす
        let mut index: usize = 4;
        let options = self.get_options();
        while options[index] != OPTION_END {
            if options[index] == option_code {
                // 目的のコードを発見したとき。データはコード、サイズ、データの順に並ぶ。
                let len = options[index + 1];
                let buf_index = index + 2;
                let v = options[buf_index..buf_index + len as usize].to_vec();
                return Some(v);
            } else if options[index] == 0 {
                // パディングなので飛ばす。
                index += 1;
            } else {
                // 目的でない他のオプション。
                index += 1;
                let len = options[index];
                index += 1;
                index += len as usize;
            }
        }
        None
    }
}
