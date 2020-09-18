#![allow(dead_code)]
#![allow(unused_variables)]
struct Foo {
    x: i32,
}

fn do_something() -> Foo {
    Foo { x: 42 }
    // 所有権は外に移動
}

fn main() {
    let foo = do_something();
    // foo は所有者になる
    // 関数のスコープの終端により、foo はドロップされる。
}
