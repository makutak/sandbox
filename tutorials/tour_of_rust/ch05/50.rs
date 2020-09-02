fn main() {
    let mut foo = 42;
    let f = &mut foo;
    println!("f: {}", f);
    let bar = *f; // 所有者の値を取得
    *f = 13; // 参照の所有者の値を設定
    println!("f: {}", f);
    println!("bar: {}", bar);
    println!("foo: {}", foo);
}
