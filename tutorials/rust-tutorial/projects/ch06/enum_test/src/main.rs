use std::net::{Ipv4Addr, Ipv6Addr};

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
enum IpAddrType {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("{:#?}", self);
    }
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:#?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
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

    let home = IpAddrType::V4(Ipv4Addr::new(127, 0, 0, 1));
    let loopback = IpAddrType::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));

    println!("{:#?}", home);
    println!("{:#?}", loopback);

    let m = Message::Write(String::from("hello"));
    m.call();

    let penny = Coin::Penny;
    let quarter = Coin::Quarter;
    println!("penny is {:#?}", value_in_cents(penny));
    println!("{:?}", value_in_cents(quarter(UsState::Alaska)));

    let five = Some(5);
    println!("five: {:#?}", five);
    let six = plus_one(five);
    println!("five plus one is: {:#?}", six);
    let none = plus_one(None);
    println!("None: {:#?}", none);

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one!"),
        3 => println!("three!"),
        5 => println!("five!"),
        7 => println!("seven!"),
        _ => (),
    }

    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("Three!!"),
        _ => (),
    }

    let some_u8_value = Some(3);
    if let Some(3) = some_u8_value {
        println!("Matched!! Three!!");
    }

    let coin = Coin::Penny;
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    println!("count is: {}", count);
}

fn route(ip_type: IpAddrKind) {
    println!("route: {:#?}", ip_type);
}
