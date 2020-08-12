use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{Shutdown, TcpStream};
use std::str;

fn recv_msg(reader: &mut BufReader<&TcpStream>) {
    for line in reader.lines() {
        println!("{}", line.unwrap());
    }
}

fn send_msg(writer: &mut BufWriter<&TcpStream>, msg: &str) {
    writer.write(msg.as_bytes()).expect("SEND FAILURE!!!");
    writer.flush().unwrap();
}

fn main() {
    let client_socket = TcpStream::connect("127.0.0.1:80");
    match client_socket {
        Err(_) => {
            println!("could not connect!!!");
        }
        Ok(stream) => {
            let request_text = "GET / HTTP/1.0\r\n\r\n";
            let mut writer = BufWriter::new(&stream);
            let mut reader = BufReader::new(&stream);
            send_msg(&mut writer, request_text);
            recv_msg(&mut reader);

            stream
                .shutdown(Shutdown::Both)
                .expect("shutdown call failed");
        }
    }
}
