use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream};
use std::str;
use std::thread;

fn recv_msg(stream: &TcpStream) {
    for line in BufReader::new(stream).lines() {
        let msg = line.unwrap_or(String::from("Error"));
        println!("{}", msg);
        send_msg(&stream, &msg)
    }
}

fn send_msg(stream: &TcpStream, msg: &str) {
    let mut writer = BufWriter::new(stream);
    writeln!(writer, "echo: {}", msg.to_string()).unwrap();
}

fn main() -> Result<(), String> {
    let listner = TcpListener::bind("127.0.0.1:54321");

    match listner {
        Ok(listner) => {
            println!("starting server...");

            for stream in listner.incoming() {
                match stream {
                    Ok(stream) => {
                        thread::spawn(move || {
                            let client = stream.peer_addr().unwrap();
                            println!("accepcted from: {}:{}", client.ip(), client.port());
                            recv_msg(&stream);
                        });
                    }
                    Err(e) => println!("accept failed!!!: {:?}", e),
                }
            }
            drop(listner);
        }
        Err(e) => println!("server start failed!!!!!!!!!!!!!: {:?}", e),
    }
    Ok(())
}
