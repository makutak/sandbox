fn main() {
    while_test();
}

fn while_test() {
    let mut number = 3;

    while number != 0 {
        println!("{}", number);

        number -= 1;
    }

    println!("LIFTOFF!!");
}
