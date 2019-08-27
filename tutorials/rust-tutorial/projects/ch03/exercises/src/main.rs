fn main() {
    assert_eq!(0, fahrenheit_to_centigrade(32));
    assert_eq!(100, fahrenheit_to_centigrade(212));
    assert_eq!(23, fahrenheit_to_centigrade(74));

    assert_eq!(0, fib(0));
    assert_eq!(1, fib(1));
    assert_eq!(1, fib(2));
    assert_eq!(55, fib(10));

    the_twelve_days_of_christmas();
}

//華氏から摂氏への変換
fn fahrenheit_to_centigrade(val: i32) -> i32 {
    (val - 32) * 5 / 9
}

fn fib(n: i32) -> i32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

fn the_twelve_days_of_christmas() {
    let ary = ["first", "second", "third"];
    for i in ary.iter() {
        println!("On the {} day of Christmas my true love sent to me", i);
    }
}
