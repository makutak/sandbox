fn main() {
    let mut s = String::from("hello"); //ここで必要なメモリが要求される
    assert_eq!("hello", s);

    s.push_str(", world!"); //メモリを更に要求する
    assert_eq!("hello, world!", s);

    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}", s1);
} //drop関数が呼ばれsのスコープが終わる
