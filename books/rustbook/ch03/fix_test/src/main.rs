use std::sync::Arc;

fn main() {
    let mut s = "Hello, World!";
    println!("{}", s);

    let string = &s;
    println!("{}", s);
}
