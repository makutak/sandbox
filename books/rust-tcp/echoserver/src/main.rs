use std::{
    env,
    error::Error,
    io::{Read, Write},
    net::TcpListener,
    str, thread,
};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let addr = &args[1];
    echo_server(addr)?;
    Ok(())
}

/**
 * エコーサーバを立ち上げる
 */
fn echo_server(address: &str) -> Result<(), Box<dyn Error>> {
    let listnner = TcpListener::bind(address)?;
    loop {
        let (mut stream, _) = listnner.accept()?;
        // スレッドを立ち上げて接続に対応する
        thread::spawn(move || {
            let mut buffer = [0u8; 1024];
            loop {
                let nbytes = stream.read(&mut buffer).unwrap();
                if nbytes == 0 {
                    return;
                }
                print!("{}", str::from_utf8(&buffer[..nbytes]).unwrap());
                stream.write_all(&buffer[..nbytes]).unwrap();
            }
        });
    }
}
