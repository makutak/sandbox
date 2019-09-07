fn main() {
    let mut s = String::from("hello"); //ここで必要なメモリが要求される
    assert_eq!("hello", s);

    s.push_str(", world!"); //メモリを更に要求する
    assert_eq!("hello, world!", s);
    move_test();
    clone_test();
    stack_copy_test();

    takes_ownership(s);
    // println!("{}", s);

    let x = 5;
    makes_copy(x);
    println!("{}", x);

    let s1 = gives_ownership();
    println!("{}", s1);

    let s2 = String::from("hello");
    println!("{}", s2);
    let s3 = takes_and_gives_back(s2);
    println!("{}", s3);

    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("change");
    change(&mut s);
    println!("{}", s);

    let r1 = &mut s;
    let r2 = &mut s;
} //drop関数が呼ばれsのスコープが終わる

fn move_test() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}", s2);
}

fn clone_test() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}

fn stack_copy_test() {
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("makes_copy: {}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

// アンド記号が参照
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!!");
}
