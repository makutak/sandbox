use std::io::Error;
use std::net::TcpListener;
use std::thread;

fn main() -> Result<(), Error> {
    let listners = TcpListener::bind("127.0.0.1:54321");
    match listners {
        Ok(listner) => {
            for stream in listner.incoming() {
                thread::spawn(move || {
                    println!("{:?}", stream);
                });
            }
            drop(listner);
        }
        Err(e) => panic!("Error!!!!!!!!!!!!!: {:?}", e),
    }

    Ok(())
}
