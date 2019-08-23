fn main() {
    while_test();
    for_test();
}

fn while_test() {
    let mut number = 3;

    while number != 0 {
        println!("{}", number);

        number -= 1;
    }

    println!("LIFTOFF!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("{}", a[index]);
        index += 1;
    }
}

fn for_test() {
    println!("for test");

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("{}", element);
    }

    for i in 1..5 {
        println!("value of {}", i);
    }
}
