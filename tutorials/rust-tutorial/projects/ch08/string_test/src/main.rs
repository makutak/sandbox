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

    let mut s = String::from("lo");
    s.push('l');
    println!("s is {}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!!");
    let s3 = s1 + &s2;
    println!("s3 is {}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s is {}", s);

    // let s1 = String::from("hello");
    // let h = s1[0];

    let len = String::from("Hola").len();
    println!("len is {}", len);

    let len = String::from("Здравствуйте").len();
    println!("len is {}", len);

    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("s is {}", s);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
