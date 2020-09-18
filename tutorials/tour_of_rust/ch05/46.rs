/*
### 所有権の移動
所有者が関数の実引数として渡されると、所有権は関数の仮引数に移動(move)する。
移動がは元の関数内のは変数は使用できなくなる。
メモリの詳細:
- 移動している間、所有者の値のスタックメモリは、関数呼び出しパラメータのスタックメモリにコピーされる
 */

struct Foo {
    x: i32,
}

fn do_something(f: Foo) {
    println!("do_something: {}", f.x);
    // f はここでドロップ
}

// これだともとの関数内でもまた使える
fn do_something2(f: &Foo) {
    println!("do_something2: {}", f.x);
}

fn main() {
    let foo = Foo { x: 42 };
    // foo の所有権は do_something に移動
    do_something(foo);
    // foo は使えなくなる
    // なので、これはエラーになる
    //println!("{:?}", foo.x);

    let bar = Foo { x: 13 };
    do_something2(&bar);
    println!("また bar を呼ぶ!!!: {}", bar.x);
}
