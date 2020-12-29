use clap::{App, Arg};
use pnet::packet::tcp::TcpFlags;
use std::collections::HashMap;
use std::fs;
use std::net::Ipv4Addr;

use std::{env, process};

#[macro_use]
extern crate log;

#[macro_use]
extern crate clap;

#[derive(Debug)]
struct PacketInfo {
    my_ipaddr: Ipv4Addr,
    target_ipaddr: Ipv4Addr,
    my_port: u16,
    maximum_port: u16,
    scan_type: ScanType,
}

#[derive(Clone, Copy, Debug)]
enum ScanType {
    Syn = TcpFlags::SYN as isize,
    Fin = TcpFlags::FIN as isize,
    Xmas = (TcpFlags::FIN | TcpFlags::URG | TcpFlags::PSH) as isize,
    Null = 0,
}

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::new("target_ipaddr")
                .about("[ipaddr]")
                .value_name("IP_ADDR")
                .index(1)
                .required(true),
        )
        .arg(
            Arg::new("scantype")
                .about("[scantype]")
                .value_name("SCAN_TYPE")
                .index(2)
                .required(true),
        )
        .get_matches();

    let target_ipaddr = matches.value_of("target_ipaddr").unwrap();
    let scan_type = matches.value_of("scantype").unwrap();

    println!("target_iparrd: {}, scan_type: {}", target_ipaddr, scan_type);

    let packet_info = {
        let contents = fs::read_to_string(".env").expect("Failed to read env file.");
        let lines: Vec<_> = contents.split('\n').collect();
        let mut map = HashMap::new();
        for line in lines {
            let elm: Vec<_> = line.split('=').map(str::trim).collect();
            if elm.len() == 2 {
                map.insert(elm[0], elm[1]);
            }
        }
        PacketInfo {
            my_ipaddr: map["MY_IPADDR"].parse().expect("invalid ipaddr"),
            target_ipaddr: target_ipaddr.parse().expect("invalid target ipaddr"),
            my_port: map["MY_PORT"].parse().expect("invalid port number"),
            maximum_port: map["MAXIMUM_PORT_NUM"]
                .parse()
                .expect("invalid maximum portnum"),
            scan_type: match scan_type {
                "sS" => ScanType::Syn,
                "sF" => ScanType::Fin,
                "sX" => ScanType::Xmas,
                "sN" => ScanType::Null,
                _ => {
                    error!("Undefined scan method, only accept [sS|sF|sN|sX|].");
                    process::exit(1);
                }
            },
        }
    };

    println!("packet info: {:?}", packet_info);
}
