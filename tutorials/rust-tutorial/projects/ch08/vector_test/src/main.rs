fn main() {
    let v = vec![1, 2, 3];
    println!("{:?}", v);

    let mut v2: Vec<i32> = Vec::new();
    println!("{:?}", v2);

    v2.push(5);
    v2.push(6);
    v2.push(7);
    v2.push(8);
    println!("{:?}", v2);
}
