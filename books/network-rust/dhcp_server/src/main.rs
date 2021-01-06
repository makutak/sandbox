use std::{
    env,
    net::{Ipv4Addr, UdpSocket},
    sync::Arc,
};

use byteorder::{BigEndian, ByteOrder};
use dhcp::{DhcpPacket, DhcpServer};

#[macro_use]
extern crate log;

mod database;
mod dhcp;
mod util;

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
    dhcp_packet.set_hlen(6); // MACアドレスのオクテット長
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

/**
 * DHCPリクエストを解析してレスポンスを返す
 */
fn dhcp_handler(
    packet: &DhcpPacket,
    soc: &UdpSocket,
    dhcp_server: &Arc<DhcpServer>,
) -> Result<(), failure::Error> {
    let message = packet
        .get_option(Code::MessageType as u8)
        .ok_or_else(|| failure::err_msg("specified option was not found"))?;

    let message_type = message[0];
    let transaction_id = BigEndian::read_u32(packet.get_xid());
    let client_macaddr = packet.get_chaddr();

    match message_type {
        // TODO: dhcp_discover_message_handler()
        DHCPDISCOVER => dhcp_discover_message_handler(transaction_id, dhcp_server, &packet, soc),
        // TODO: dhcp_request_message_handler_responded_to_offer()
        DHCPREQUEST => {
            println!("DHCPREQUEST");
            Err(failure::err_msg("not yet implimented"))
        }
        // TODO: dhcp_release_message_handler()
        DHCPRELEASE => {
            println!("DHCPRELEASE");
            Err(failure::err_msg("not yet implimented"))
        }
        _ => {
            let msg = format!(
                "{:?}: received unimplemented message, message_type: {}",
                transaction_id, message_type
            );
            Err(failure::err_msg(msg))
        }
    }
}

/**
 * DISCOVER メッセージを受信したときのハンドラ。
 * 利用できるアドレスを選択してDHCPOFFERメッセージを返却する。
 */
fn dhcp_discover_message_handler(
    xid: u32,
    dhcp_server: &Arc<DhcpServer>,
    received_packet: &DhcpPacket,
    soc: &UdpSocket,
) -> Result<(), failure::Error> {
    info!("{:?} received DHCPDISCOVER", xid);

    let ip_to_be_leased = select_lease_ip(&dhcp_server, &received_packet)?;

    // 決定したIPアドレスでDHCPパケットの作成
    let dhcp_packet = make_dhcp_packet(&received_packet, &dhcp_server, DHCPOFFER, ip_to_be_leased)?;
    util::send_dhcp_broadcast_response(soc, dhcp_packet.get_buffer())?;

    info!("{:?}: sent DHCPOFFER", xid);
    Ok(())
}

/**
 * 利用可能なIPアドレスを選ぶ。
 * 1. 以前そのクライアントにリースされたIPアドレス（解放されたものも含む）
 * 2. クライアントから要求されたIPアドレス
 * 3. アドレスプール
 * の優先順位で利用可能なIPアドレスを返却する。
 */
fn select_lease_ip(
    dhcp_server: &Arc<DhcpServer>,
    received_packet: &DhcpPacket,
) -> Result<Ipv4Addr, failure::Error> {
    // クリティカルセッションを短くするためブロックに入れる
    let con = dhcp_server.db_connection.lock().unwrap();
    // DBから以前割り当てたれたIPアドレスがあればそれを返す。
    if let Some(ip_from_used) = database::select_entry(&con, received_packet.get_chaddr())? {
        // IPアドレスが重複してないか
        // .env に記載sれたネットワークアドレスの変更があったときのために、
        // 現在のネットワークに含まれているかを合わせて確認する
        if dhcp_server.network_addr.contains(ip_from_used)
            && util::is_ipaddr_available(ip_from_used).is_ok()
        {
            return Ok(ip_from_used);
        }
    }

    // Requested Ip Addr オプションがあり、利用可能ならばそのIPアドレスを返却。
    if let Some(ip_to_be_leased) =
        obtain_available_ip_from_requested_option(dhcp_server, received_packet)
    {
        return Ok(ip_to_be_leased);
    }

    // アドレスプールからの取得
    while let Some(ip_addr) = dhcp_server.pick_available_ip() {
        if util::is_ipaddr_available(ip_addr).is_ok() {
            return Ok(ip_addr);
        }
    }

    // 利用できるIPアドレスが取得できなかった場合
    Err(failure::err_msg("Could not obtain available ip address."))
}

/**
 * オプションに'requested IP address'があり、利用可能ならばそれを返す。
 */
fn obtain_available_ip_from_requested_option(
    dhcp_server: &Arc<DhcpServer>,
    received_packet: &DhcpPacket,
) -> Option<Ipv4Addr> {
    let ip = received_packet.get_option(Code::RequestedIpAddress as u8)?;
    let requested_ip = util::u8_to_ipv4addr(&ip)?;
    // アドレスプールからの検索
    let ip_from_pool = dhcp_server.pick_specified_ip(requested_ip)?;
    if util::is_ipaddr_available(ip_from_pool).is_ok() {
        return Some(requested_ip);
    }
    None
}
