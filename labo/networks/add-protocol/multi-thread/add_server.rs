use std::net::TcpListener;

fn main() {
    let listner = match TcpListener::bind("127.0.0.1:54321") {
        Ok(listner) => listner,
        Err(e) => panic!("bind failed!! {:?}", e),
    };

    println!("starting server...");

    for stream in listner.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted from {}", stream.peer_addr().unwrap().to_string());
            }
            Err(e) => panic!("socket accepted failed!! {:?}", e),
        };
    }
}
