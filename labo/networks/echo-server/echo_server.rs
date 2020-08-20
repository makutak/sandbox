use std::io::Error;
use std::net::TcpListener;
use std::thread;

fn main() -> Result<(), Error> {
    let listners = TcpListener::bind("127.0.0.1:54321")?;
    println!("starting server...");
    for streams in listners.incoming() {
        match streams {
            Ok(stream) => {
                thread::spawn(move || {
                    let client = stream.peer_addr().unwrap();
                    println!("accepcted from: {}:{}", client.ip(), client.port());
                });
            }
            Err(e) => panic!("{:?}", e),
        }
    }
    Ok(())
}
