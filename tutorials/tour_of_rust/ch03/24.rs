fn main() {
    // スタティックメソッドでStringインスタンスを作成する。
    let s = String::from("Hello world!");
    // インスタンスを使ってメソッドを呼び出す。
    println!("{} is {} characters long.", s, s.len());
}
