#![allow(dead_code)]
#![allow(unused_variables)]

/*
構造体がドロップされると、まず構造体自体がドロップされ、
その次にその子要素が個別に削除される。
 */

struct Bar {
    x: i32,
}

struct Foo {
    bar: Bar,
}

fn main() {
    let foo = Foo { bar: Bar { x: 42 } };
    println!("{}", foo.bar.x);
    // foo が最初にドロップ
    // 次にfoo.barがドロップ
}
