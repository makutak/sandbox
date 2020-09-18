/*
format! マクロはパラメータ化された文字列を定義して、
値をどこにどのように配置するかをプレースホルダ（例：{}）
で定義することで文字列を作成することができる。

format! は println! と同じパラメータ付きの文字列を使用する。
 */

fn main() {
    let a = 42;
    let f = format!("secret to life: {}", a);
    println!("{}", f);
}
