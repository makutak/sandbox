fn main() {
    let mut s = String::from("hello");
    assert_eq!("hello", s);

    s.push_str(", world!");
    assert_eq!("hello, world!", s);
}
