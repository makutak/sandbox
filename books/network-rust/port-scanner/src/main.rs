use clap::{App, Arg};

use std::env;

#[macro_use]
extern crate log;

#[macro_use]
extern crate clap;

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
                .value_name("SCAN TYPE")
                .index(2)
                .required(true),
        )
        .get_matches();

    let target_ipaddr = matches.value_of("target_ipaddr").unwrap();
    let scan_type = matches.value_of("scantype").unwrap();

    println!("target_iparrd: {}, scan_type: {}", target_ipaddr, scan_type);
}
