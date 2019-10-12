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

    let sum = 5 + 10;
    println!("sum: {}", sum);

    let diff = 95.5 - 4.3;
    println!("diff: {}", diff);

    let prod = 4 * 30;
    println!("prod: {}", prod);

    let quot = 56.7 / 32.2;
    println!("quot: {}", quot);

    let rem = 43 % 5;
    println!("rem: {}", rem);

    let t = true;
    println!("t: {}", t);

    let f: bool = false;
    println!("f: {}", f);

    let c = 'z';
    println!("c: {}", c);

    let z = 'ℤ';
    println!("z: {}", z);

    let heart_eyed_cat = '😻';
    println!("heart_eyed_cat, {}", heart_eyed_cat);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, c) = tup;
    println!("a: {}, b: {}, c: {}", a, b, c);

    let x = tup.0;
    println!("x: {}", x);

    let y = tup.1;
    println!("y: {}", y);

    let z = tup.2;
    println!("z: {}", z);

    let ary = [1, 2, 3, 4, 5];
    let first = ary[0];
    let second = ary[1];
    println!("first: {}, second: {}", first, second);

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("months: {:?}", months);

    // let index = 10;
    // println!("{}", ary[index]);

    println!("##############################################");
}
