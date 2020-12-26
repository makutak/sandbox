use clap::{App, Arg};
use pnet::datalink;
use std::env;

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

    println!("{}", interface);
}
