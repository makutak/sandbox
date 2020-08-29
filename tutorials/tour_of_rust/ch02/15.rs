fn main() {
    let mut x = 0;
    loop {
        x += 1;
        if x == 42 {
            break;
        }
        println!("now...: {}", x);
    }
    println!("{}", x);
}
