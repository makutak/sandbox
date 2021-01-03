use std::{
    env,
    net::{Ipv4Addr, UdpSocket},
    sync::Arc,
};

use dhcp::{DhcpPacket, DhcpServer};

#[macro_use]
extern crate log;

mod dhcp;

const HTYPE_ETHER: u8 = 1;

const DHCP_SIZE: usize = 400;

enum Code {
    MessageType = 53,
    IPAddressLeaseTime = 51,
    ServerIdentifier = 54,
    RequestedIpAddress = 50,
    SubnetMask = 1,
    Router = 3,
    DNS = 6,
    End = 255,
}

const DHCPDISCOVER: u8 = 1;
const DHCPOFFER: u8 = 2;
const DHCPREQUEST: u8 = 3;
const DHCPACK: u8 = 5;
const DHCPNAK: u8 = 6;
const DHCPRELEASE: u8 = 7;

const BOOTREQUEST: u8 = 1;
const BOOTREPRY: u8 = 2;

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let server_socket = UdpSocket::bind("0.0.0.0:67").expect("Failed to bind socket");
    server_socket.set_broadcast(true).unwrap();

    loop {
        let mut recv_buf = [0u8; 1024];
        match server_socket.recv_from(&mut recv_buf) {
            Ok((size, src)) => {
                debug!("received data from {}, size: {}", src, size);
            }
            Err(e) => {
                error!("Could not receive a datagram: {}", e);
            }
        }
    }
}

/**
 * DHCPのパケットを作成して返す
 */
fn make_dhcp_packet(
    received_packet: &DhcpPacket,
    dhcp_server: &Arc<DhcpServer>,
    message_type: u8,
    ip_to_be_leased: Ipv4Addr,
) -> Result<DhcpPacket, failure::Error> {
    // パケットの本体となるバッファ。ヒープに確保する。
    let buffer = vec![0u8; DHCP_SIZE];
    let mut dhcp_packet = DhcpPacket::new(buffer).unwrap();

    // 各種フィールドの設定
    dhcp_packet.set_op(BOOTREPRY);
    dhcp_packet.set_htype(HTYPE_ETHER);
    dhcp_packet.set_xid(6); // MACアドレスのオクテット長
    dhcp_packet.set_xid(received_packet.get_xid());
    if message_type == DHCPACK {
        dhcp_packet.set_ciaddr(received_packet.get_ciaddr());
    }
    dhcp_packet.set_yiaddr(ip_to_be_leased);
    dhcp_packet.set_flags(received_packet.get_flags());
    dhcp_packet.set_giaddr(received_packet.get_giaddr());
    dhcp_packet.set_chaddr(received_packet.get_chaddr());

    //各種オプションの設定
    let mut cursor = dhcp::OPTIONS;
    dhcp_packet.set_magic_cookie(&mut cursor);
    dhcp_packet.set_option(
        &mut cursor,
        Code::MessageType as u8,
        1,
        Some(&[message_type]),
    );
    dhcp_packet.set_option(
        &mut cursor,
        Code::IPAddressLeaseTime as u8,
        4,
        Some(&dhcp_server.lease_time),
    );
    dhcp_packet.set_option(
        &mut cursor,
        Code::ServerIdentifier as u8,
        4,
        Some(&dhcp_server.server_address.octets()),
    );
    dhcp_packet.set_option(
        &mut cursor,
        Code::SubnetMask as u8,
        4,
        Some(&dhcp_server.subnet_mask.octets()),
    );
    dhcp_packet.set_option(
        &mut cursor,
        Code::Router as u8,
        4,
        Some(&dhcp_server.default_gateway.octets()),
    );
    dhcp_packet.set_option(
        &mut cursor,
        Code::DNS as u8,
        4,
        Some(&dhcp_server.dns_server.octets()),
    );
    dhcp_packet.set_option(&mut cursor, Code::End as u8, 0, None);
    Ok(dhcp_packet)
}
