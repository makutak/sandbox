fn main() {
    println!("Hello, world!");

    another_function(5, 10);

    statement_test();
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is {}: ", x);
    println!("The value of y is {}: ", y);
}

fn statement_test() {
    let a = 5;
    println!("a: {}: ", a);

    let x = (let y = 6);
}
