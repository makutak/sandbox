use clap::{App, Arg};
use std::env;

#[macro_use]
extern crate log;

mod tcp_client;
mod tcp_server;
mod udp_server;

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let matches = App::new("Echo Server")
        .version("1.0.0")
        .author("Fix me")
        .about("TCP/UDP Echo Server")
        .arg(
            Arg::new("protocol")
                .about("[tcp|udp]")
                .value_name("PROTOCOL")
                .index(1)
                .required(true),
        )
        .arg(
            Arg::new("role")
                .about("[server|client]")
                .value_name("ROLE")
                .index(2)
                .required(true),
        )
        .arg(
            Arg::new("address")
                .about("[addr:port]")
                .value_name("ADDRESS")
                .index(3)
                .required(true),
        )
        .get_matches();

    let protocol = matches.value_of("protocol").unwrap();
    let role = matches.value_of("role").unwrap();
    let address = matches.value_of("address").unwrap();

    match protocol {
        "tcp" => match role {
            "server" => {
                tcp_server::serve(address).unwrap_or_else(|e| error!("{}", e));
            }
            "client" => {
                tcp_client::connect(address).unwrap_or_else(|e| error!("{}", e));
            }
            _ => {
                missing_role();
            }
        },
        "udp" => match role {
            "server" => {
                udp_server::serve(address).unwrap_or_else(|e| error!("{}", e));
            }
            "client" => {
                // UDPクライアントの呼び出し
            }
            _ => {
                missing_role();
            }
        },

        _ => {
            error!("Please specify tcp or udp on the 1st argument.");
            std::process::exit(1);
        }
    }
}

fn missing_role() {
    error!("Please specify server or client on the 2nd argument.");
    std::process::exit(1);
}
