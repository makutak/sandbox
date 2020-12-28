use clap::{App, Arg};
use pnet::datalink;
use pnet::datalink::Channel::Ethernet;
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ipv6::Ipv6Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::udp::UdpPacket;
use pnet::packet::Packet;

use std::env;

#[macro_use]
extern crate log;

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let matches = App::new("Packet Capture")
        .version("1.0.0")
        .author("Fix me")
        .about("Packagt Capture")
        .arg(
            Arg::new("interface")
                .about("interface name")
                .value_name("INTERFACE_NAME")
                .index(1)
                .required(true),
        )
        .get_matches();

    let interface_name = matches.value_of("interface").unwrap();

    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .find(|iface| iface.name == interface_name)
        .expect("Failed to get interface");

    let (_tx, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!("Failed to create datalink channel {}", e),
    };

    loop {
        match rx.next() {
            Ok(frame) => {
                // 受信データからイーサネットフレームの構築
                let frame = EthernetPacket::new(frame).unwrap();
                match frame.get_ethertype() {
                    EtherTypes::Ipv4 => {
                        ipv4_handler(&frame);
                    }
                    EtherTypes::Ipv6 => {
                        ipv6_handler(&frame);
                    }
                    _ => {
                        info!("Not an IPv4 or IPv6 packet");
                    }
                }
            }
            Err(e) => {
                error!("failed to read: {}", e);
            }
        }
    }
}

/**
 * IPv4パケットを構築し次のレイヤのハンドラを呼び出す
 */
fn ipv4_handler(ethernet: &EthernetPacket) {
    if let Some(packet) = Ipv4Packet::new(ethernet.payload()) {
        match packet.get_next_level_protocol() {
            IpNextHeaderProtocols::Tcp => {
                println!("tcp");
            }
            IpNextHeaderProtocols::Udp => {
                println!("udp");
            }
            _ => {
                info!("Not a TCP or UDP packet.");
            }
        }
    }
}

/**
 * IPv6パケットを構築し次のレイヤのハンドラを呼び出す
 */
fn ipv6_handler(ethernet: &EthernetPacket) {
    if let Some(packet) = Ipv6Packet::new(ethernet.payload()) {
        match packet.get_next_header() {
            IpNextHeaderProtocols::Tcp => {
                println!("tcp");
            }
            IpNextHeaderProtocols::Udp => {
                println!("udp");
            }
            _ => {
                info!("not a TCP or UDP packet.")
            }
        }
    }
}
