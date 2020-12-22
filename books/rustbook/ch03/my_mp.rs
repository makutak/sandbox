use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    tx.send("Hello, world!".to_string())
        .expect("reader disconnected!!");
    let recv = rx.recv().expect("sender disconnected!!");
    println!("{}", recv);
}
