/*
 * Rustでは可変な参照が1つだけか、不変な参照が複数かのどちらかが許可される。両方を同時には使用できない。
   -> データ競合を防ぐため。
 * 参照は所有者よりも長く存在してはならない。
   -> 存在しないデータへの参照(C言語で言うダングリングポインタ)による誤動作を防ぐため。
*/

struct Foo {
    x: i32,
}

fn do_something(f: &mut Foo) {
    f.x += 1;
    // f への可変な参照はここでドロップ
}

fn main() {
    let mut foo = Foo { x: 42 };
    do_something(&mut foo);
    println!("foo.x: {}", foo.x);
    // 関数 do_something で可変な参照はドロップされるため、
    // 別の参照を作ることが可能
    do_something(&mut foo);
    println!("foo.x: {}", foo.x);
    // foo はここでドロップ
}
