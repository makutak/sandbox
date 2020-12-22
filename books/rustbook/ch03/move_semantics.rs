#[derive(Debug)]
struct Color {
    r: i32,
    g: i32,
    b: i32,
}

fn main() {
    let a = Color {
        r: 255,
        g: 255,
        b: 255,
    };
    println!("a: {:?}, address: {:p}", a, &a);

    let b = a;
    //println!("a: {:?}", a); //所有権が移っているので使用できない
    println!("b: {:?}, address: {:p}", b, &b);
}
