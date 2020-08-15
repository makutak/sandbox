/*
 * 関数に戻り値の型が指定されていない場合、unit と呼ばれる空のタプルを返します。
 * 空のタプルは () と表記します。
 */

fn make_nothing() -> () {
    return ();
}

// 戻り値は()と推論される
fn make_nothing2() {
    // この関数は戻り値が指定されていないため()を返す
}

fn say_hello() {
    //hello と出力するが何も返さない
    println!("hello");
}

fn main() {
    let a = make_nothing();
    let b = make_nothing2();
    let said = say_hello();

    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", said);
}
