use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};

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

fn recv_msg(mut sock: &TcpStream, chunk_len: usize) -> Vec<u8> {
    let mut buf = vec![0; chunk_len];
    match sock.read_to_end(&mut buf) {
        Ok(_) => (),
        Err(e) => panic!("socket read failed: {:?}", e),
    };

    buf
}

fn main() {
    let stream = match TcpStream::connect("127.0.0.1:8080") {
        Ok(stream) => stream,
        Err(e) => panic!("connection failed!! :{:?}", e),
    };

    let request_text = "GET / HTTP/1.0\r\n\r\n";
    let request_bytes = request_text.as_bytes().to_vec();
    send_msg(&stream, &request_bytes);

    let received_bytes = recv_msg(&stream, 1024);
    let received_text = std::str::from_utf8(&received_bytes).unwrap();
    println!("{}", received_text);

    match stream.shutdown(Shutdown::Both) {
        Ok(_) => (),
        Err(e) => panic!("shutdown failed!! {:?}", e),
    };
}
