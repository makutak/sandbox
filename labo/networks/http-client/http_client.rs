use std::net::TcpStream;

fn send_msg() {}

fn recv_msg() {}

fn main() {
    let stream = TcpStream::connect("127.0.0.1:80");
    match stream {
        Err(_) => {
            println!("connection error!");
        }
        Ok(stream) => {
            println!("connect!!");
        }
    }
}
