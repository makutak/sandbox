use clap::{App, Arg};
use pnet::datalink;
use pnet::datalink::Channel::Ethernet;
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
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
                        println!("ipv4");
                    }
                    EtherTypes::Ipv6 => {
                        println!("ipv6");
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
