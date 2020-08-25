use std::convert::TryInto;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};

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
    let stream = match TcpStream::connect("127.0.0.1:54321") {
        Ok(stream) => stream,
        Err(e) => panic!("connection failed!! {:?}", e),
    };

    let operand1: i32 = 1000;
    let operand2: i32 = 2000;
    println!("operand1: {}, operand2: {}", operand1, operand2);
    let mut request_msg = operand1.to_be_bytes().to_vec();
    request_msg.append(&mut operand2.to_be_bytes().to_vec());
    send_msg(&stream, &request_msg);
    println!("sent {:?}", request_msg);
    let received_msg = recv_msg(&stream, 8);
    println!("{:?}", received_msg);

    let result = i64::from_be_bytes((&received_msg[..]).try_into().unwrap());
    println!("{:?}", result);

    match stream.shutdown(Shutdown::Both) {
        Ok(_) => (),
        Err(e) => panic!("shutdown failed!! {:?}", e),
    };
}
