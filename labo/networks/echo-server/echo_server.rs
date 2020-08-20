use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn recv(reader: &mut BufReader<&TcpStream>) {
    for line in reader.lines() {
        let input = match line {
            Ok(line) => line,
            Err(e) => String::from(format!("read error: {:?}", e)),
        };

        println!("input: {:?}", input);
    }
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
                            let mut reader = BufReader::new(&stream);
                            recv(&mut reader);
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
