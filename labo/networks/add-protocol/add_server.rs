use std::net::{TcpListener, TcpStream};

fn recv_msg(sock: &TcpStream, total_msg_size: i64) {
    // これまでに受信できたバイト数
    let mut total_recv_size = 0;

    //while total_recv_size < total_msg_size {}
}

fn main() {
    let listner = TcpListener::bind("127.0.0.1:54321").expect("server start failed!");
    println!("starting server...");

    let (_client_socket, addr) = listner.accept().expect("accept failed!");
    println!("accepted from: {}", addr);
}
