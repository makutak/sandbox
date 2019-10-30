fn main() {
    let v = vec![1, 2, 3];
    println!("{:?}", v);

    let mut v: Vec<i32> = Vec::new();
    println!("{:?}", v);

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("{:?}", v);

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("{}", third);
    let third: Option<&i32> = v.get(2);
    println!("{:?}", third);

    let v = vec![1, 2, 3, 4, 5];

    // let does_not_exist = &v4[100];
    // println!("{:?}", does_not_exist);

    let does_not_exist = v.get(100);
    println!("{:?}", does_not_exist);

    // let v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // v.push(6);

    println!("for loop...");
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    println!("{:?}", v);
    for i in &mut v {
        *i += 50;
    }
    println!("{:?}", v);
}
