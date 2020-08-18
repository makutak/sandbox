use std::io::Error;
use std::net::TcpListener;

fn main() -> Result<(), Error> {
    let server_socket = TcpListener::bind("127.0.0.1:80");

    match server_socket {
        Ok(v) => println!("{:?}", v),
        Err(e) => panic!("Bind failed!!!: {:?}", e),
    }

    Ok(())
}
