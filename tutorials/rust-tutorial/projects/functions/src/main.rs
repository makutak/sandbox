fn main() {
    println!("Hello, world!");

    another_function(5, 10);

    statement_test();

    let x = five();
    println!("five(): {}", x);

    let added = plus_one(100_0);
    println!("added: {}", added);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is {}: ", x);
    println!("The value of y is {}: ", y);
}

fn statement_test() {
    let a = 5;
    println!("a: {}: ", a);

    let y = {
        let x = 3;
        x + 1
    };

    println!("y: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
