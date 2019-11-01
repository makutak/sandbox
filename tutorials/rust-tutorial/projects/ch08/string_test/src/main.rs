fn main() {
    let mut s = String::new();
    println!("{}", s);

    let data = "initial contents";
    println!("data: {}", data);

    let s = data.to_string();
    println!("s: {}", s);

    let s = "initial contents".to_string();
    println!("s: {}", s);
}
