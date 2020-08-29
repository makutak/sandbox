/*
 * Option/Result を使うのはちょっとしたコードを書くのには厄介である。
 * => Option と Result には、unwrap という関数があり、手っ取り早く値を取得するのには便利である。
 * unwrap は以下のことを行う。
 * 1. Option/Result 内の値を取得する。
 * 2. 列挙型が None/Errの場合、panic! する。
 * 良い Rustacean であるためには可能な限り適切に match を使おう。
 */

fn do_something_that_might_fail(i: i32) -> Result<f32, String> {
    if i == 42 {
        Ok(13.0)
    } else {
        Err(String::from("正しい値ではありません"))
    }
}

fn main() -> Result<(), String> {
    // 簡潔ですが、値が存在することを仮定しており、
    // すぐにだめになる可能性があります。
    let v = do_something_that_might_fail(42).unwrap();
    println!("発見 {}", v);

    // パニックする！！
    let v = do_something_that_might_fail(1).unwrap();
    println!("発見 {}", v);

    Ok(())
}
