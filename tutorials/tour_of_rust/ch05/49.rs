/*
`&mut` 演算子を使えば、リソースへの変更可能なアクセスを借用することもできる。
リソースの所有者は可変な借用の間は移動や変更ができない。
データ競合を防止するため、Rustでは同時に2つの変数から値を変更することはできない。
 */

struct Foo {
    x: i32,
}

fn do_something(f: Foo) {
    println!("{}", f.x);
    // f はここでドロップ
}

fn main() {
    let mut foo = Foo { x: 42 };
    let f = &mut foo;

    // 失敗: do_something(foo) はここでエラー
    // foo は可変(f)に借用されており、所有権の移動はできないため

    println!("f.x: {}", f.x);

    // 失敗: foo.x = 13 はここでエラー
    // foo は可変(f)に借用されている間は変更できないため

    f.x = 13;

    println!("f.x: {}", f.x);
    // f はここから先では使用されてないため、ここでpドロップされる。

    println!("foo.x: {}", foo.x);

    // 可変な借用(f) はドロップされているため変更可能
    foo.x = 7;
    println!("foo.x: {}", foo.x);

    do_something(foo);
}
