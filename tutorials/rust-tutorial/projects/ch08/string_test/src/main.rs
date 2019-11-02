fn main() {
    let mut s = String::new();
    println!("{}", s);

    let data = "initial contents";
    println!("data: {}", data);

    let s = data.to_string();
    println!("s: {}", s);

    let s = "initial contents".to_string();
    println!("s: {}", s);

    let s = String::from("initial contents");
    println!("s: {}", s);

    let mut s = String::from("foo");
    println!("s: {}", s);

    s.push_str(" bar");
    println!("s: {}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);
}
