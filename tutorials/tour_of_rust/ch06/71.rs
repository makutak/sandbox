/*
多くの型は to_string を用いて文字列に変換することができる。

ジェネリック関数 parse を用いることで文字列や文字列リテラルを型付きの値に変換できる。
この関数は失敗する可能性があるので、 Resultを返す。
 */

fn main() -> Result<(), std::num::ParseIntError> {
    let a = 42;
    let a_string = a.to_string();
    println!("to_stringed: {}", a);
    let b = a_string.parse::<i32>()?;
    println!("parsed: {}", b);
    Ok(())
}
