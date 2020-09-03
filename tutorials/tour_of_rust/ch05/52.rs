struct Foo {
    x: i32,
}

fn do_something(a: &Foo) -> &i32 {
    return &a.x;
}

fn main() {
    let mut foo = Foo { x: 42 };
    println!("foo.x: {:p}, {}", &foo.x, foo.x);
    let x = &mut foo.x;
    println!("    x: {:p}, {}", x, x);
    *x = 13;
    println!("    x: {:p}, {}", x, x);
    println!("foo.x: {:p}, {}", &foo.x, foo.x);

    let y = do_something(&foo);
    println!("{}, {:p}", y, y);
}
