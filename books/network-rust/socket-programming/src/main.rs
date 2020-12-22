use clap::{App, Arg};
use std::env;

#[macro_use]
extern crate log;

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

    match matches.value_of("protocol") {
        Some(p) => println!("protocol: {}", p),
        None => println!("No protocol specified"),
    }

    match matches.value_of("role") {
        Some(r) => println!("role: {}", r),
        None => println!("No role specified"),
    }

    match matches.value_of("address") {
        Some(a) => println!("address: {}", a),
        None => println!("No address specified"),
    }
}
