use std::io::Write;
use std::net::{Shutdown, TcpStream};

fn send_msg() {}

fn recv_msg() {}

fn main() {
    let client_socket = TcpStream::connect("127.0.0.1:80");
    match client_socket {
        Err(_) => {
            println!("could not connect!!!");
        }
        Ok(mut stream) => {
            println!("connect!!!");
            let request_text = "GET / HTTP/1.0\r\n\r\n";
            stream
                .write(request_text.as_bytes())
                .expect("failed to write!!");

            stream
                .shutdown(Shutdown::Both)
                .expect("shutdown call failed");
        }
    }
}
