#![allow(dead_code)]
#![allow(unused_variables)]

/*
Rustではスコープの終わりをリソースのデストラクトと開放の場所として使用する。
このデストラクトのことをoドロップ(drop)と呼ぶ。
 */

struct Foo {
    x: i32,
}

fn main() {
    let foo_a = Foo { x: 42 };
    let foo_b = Foo { x: 13 };

    println!("{}", foo_a.x);

    println!("{}", foo_b.x);
    // foo_b はここでドロップ
    // foo_a はここでドロップ
}
