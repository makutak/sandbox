/*
 * main は Result を返すことができます
 */

fn do_somethins_that_might_fail(i: i32) -> Result<f32, String> {
    if i == 42 {
        Ok(13.0)
    } else {
        Err(String::from("正しい値ではありません"))
    }
}

fn main() -> Result<(), String> {
    let result = do_somethins_that_might_fail(42);

    match result {
        Ok(v) => println!("発見: {}", v),
        Err(_e) => {
            // エラーをうまく処理

            // 何が起きたのかを説明する新しい Err を main から返します！
            return Err(String::from("main で何か問題が置きました！"));
        }
    }

    Ok(())
}
