use std::io::Read;
use std::net::{TcpListener, TcpStream};

fn recv_msg(mut sock: &TcpStream, total_msg_size: usize) -> Vec<u8> {
    // これまでに受信できたバイト数
    let mut total_recv_size = 0;

    let result = loop {
        let buf_size = total_msg_size - total_recv_size;
        let mut buf = vec![0; buf_size];
        //let received_chunk = sock.take(buf_size as u64).read_to_end(&mut buf);
        let received_chunk = sock.read_exact(&mut buf);
        println!("received_chunk: {:?}", received_chunk);
        println!("buf: {:?}", buf);

        if total_recv_size < total_msg_size {
            break buf;
        }

        if buf.len() == 0 {
            // 1バイトも読めなかったときはソケットの接続が終了している
            panic!("socket connection broken");
        }

        total_recv_size += buf.len();
    };

    result
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
    let received_msg = recv_msg(&client_socket, 8);
    println!("received_msg: {:?}", received_msg);
}
