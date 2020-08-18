use std::net::TcpListener;

fn main() -> Result<(), String> {
    let server_socket = TcpListener::bind("127.0.0.1:8080");

    match server_socket {
        Ok(v) => println!("{:?}", v),
        Err(_e) => {
            return Err(format!("bind failed!!!: {}", _e));
        }
    }

    Ok(())
}
