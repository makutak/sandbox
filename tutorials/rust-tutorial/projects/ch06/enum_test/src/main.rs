#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum IpAddrEnum {
    V4(String),
    V6(String),
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("{:#?}", four);
    println!("{:#?}", six);

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    println!("{:#?}", home);

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("{:#?}", loopback);

    let home = IpAddrEnum::V4(String::from("127.0.0.1"));
    let loopback = IpAddrEnum::V6(String::from("::1"));

    println!("{:#?}", home);
    println!("{:#?}", loopback);
}

fn route(ip_type: IpAddrKind) {
    println!("route: {:#?}", ip_type);
}
