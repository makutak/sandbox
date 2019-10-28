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

    let v3 = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v3[2];
    println!("{}", third);
    let third: Option<&i32> = v.get(2);
    println!("{:?}", third);

    let v4 = vec![1, 2, 3, 4, 5];

    let does_not_exist = &v4[100];
    println!("{:?}", does_not_exist);

    let does_not_exist = v4.get(100);
    println!("{:?}", does_not_exist);
}
