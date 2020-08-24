use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};

fn recv_msg(mut sock: &TcpStream, total_msg_size: usize) -> Vec<u8> {
    // これまでに受信できたバイト数
    let mut total_recv_size = 0;

    let result = loop {
        let buf_size = total_msg_size - total_recv_size;
        let mut buf = vec![0; buf_size];
        //let received_chunk = sock.take(buf_size as u64).read_to_end(&mut buf);
        match sock.read_exact(&mut buf) {
            Ok(_) => (),
            Err(e) => panic!("socket read failed: {:?}", e),
        };

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

fn send_msg(mut sock: &TcpStream, msg: &Vec<u8>) {
    let mut total_sent_len = 0;

    let total_msg_len = msg.len();

    while total_sent_len < total_msg_len {
        let sent_len = match sock.write(&msg[total_sent_len..]) {
            Ok(v) => v,
            Err(e) => panic!("sent_len write failed!!: {:?}", e),
        };

        if sent_len == 0 {
            panic!("socket connection broken");
        }

        total_sent_len += sent_len;
    }
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
    println!(
        "{} {} {} {}",
        received_msg[0], received_msg[1], received_msg[2], received_msg[3]
    );

    // バイト列を2つの32ビットの整数として解釈する
    // TODO: イケてないので何とかする
    let operand1 = u32::from_be_bytes([
        received_msg[0],
        received_msg[1],
        received_msg[2],
        received_msg[3],
    ]);

    let operand2 = u32::from_be_bytes([
        received_msg[4],
        received_msg[5],
        received_msg[6],
        received_msg[7],
    ]);

    println!("received_msg: {:?}", received_msg);
    println!("operand1: {}, operand2: {}", operand1, operand2);
    // 計算した値を64ビットの整数としてネットワークバイトオーダーのバイト列に変換する
    let result: u64 = (operand1 + operand2).into();
    let result_msg = result.to_be_bytes().to_vec();

    send_msg(&client_socket, &result_msg);
    println!("{:?}", result_msg);

    match client_socket.shutdown(Shutdown::Both) {
        Ok(_) => (),
        Err(e) => panic!("{:?}", e),
    };
}
