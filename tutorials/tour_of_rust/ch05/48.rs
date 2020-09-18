/*
参照は、`&` 演算子を使ってリソースへのアクセスを借用できるようにしてくれる。
参照も他のリソースと同様にドロップされる。
 */
struct Foo {
    x: i32,
}

fn main() {
    let foo = Foo { x: 42 };
    println!("foo.x: {}", foo.x);
    // ここで所有権が移動
    let f = foo;
    println!("f.x: {}", f.x);

    // これは怒られる
    //println!("foo.x: {}", foo.x);

    let bar = Foo { x: 42 };
    println!("bara.x : {}", bar.x);
    //参照による所有権の借用
    let b = &bar;
    println!("b.x: {}", b.x);

    // 参照元は使える
    println!("bar.x: {}", bar.x);

    // ここでドロップ
}
