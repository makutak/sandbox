fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of x is: {}", x);

    const MAX_POINTS: u32 = 100_000;
    println!("The MAX_POINTS is: {}", MAX_POINTS);

    shadowing();
    data_types();
}

fn shadowing() {
    println!("##########################");
    println!("This is shadowing test");

    let x = 5;
    println!("The value of x is: {}", x);

    let x = x * 2;
    println!("The value of x is: {}", x);

    let x = x * 100_0;
    println!("The value of x is: {}", x);

    let spaces = "    ";
    println!("The value of spaces is {}: ", spaces);

    let spaces = spaces.len();
    println!("The length of spaces is {}: ", spaces);

    println!("##########################");
}

fn data_types() {
    println!("##############################################");
    println!("float test.");

    let x = 2.0;
    println!("{} is f64", x);

    let y: f32 = 3.0;

    println!("{} is f32", y);
    println!("##############################################");
}
