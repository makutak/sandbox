struct Foo {
    x: i32,
}

fn do_something_0(foo: &Foo) -> i32 {
    return foo.x;
}

// 引数 foo と戻り値はライフタイムを共有
fn do_something<'a>(foo: &'a Foo) -> &'a i32 {
    return &foo.x;
}

fn main() {
    let mut foo = Foo { x: 42 };
    println!("foo.x address: {:p}", &foo.x);
    let x = &mut foo.x;
    *x = 13;
    // x はここでドロップされるため、不変な参照が作成可能

    let y = do_something_0(&foo);
    println!("y address: {:p}, y: {}", &y, y);

    let z = do_something(&foo);
    println!("z address: {:p}, z: {}", z, z);

    // z はここでドロップされるため
    // foo はここでドロップされる
}
