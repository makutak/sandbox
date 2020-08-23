use std::net::{TcpListener, TcpStream};

fn recv_msg(sock: &TcpStream, total_msg_size: i64) {
    // これまでに受信できたバイト数
    let mut total_recv_size = 0;

    //while total_recv_size < total_msg_size {}
}

fn main() {
    let listner = match TcpListener::bind("127.0.0.1:54321") {
        Ok(listner) => listner,
        Err(e) => panic!("server start failed!!!: {:?}", e),
    };

    println!("starting server...");

    let (client_socket, addr) = match listner.accept() {
        Ok((socket, addr)) => (socket, addr),
        Err(e) => panic!("socket accepted failed!!!: {:?}", e),
    };

    println!("accepted from: {}", addr);
}
