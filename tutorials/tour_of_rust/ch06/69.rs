/*
concat と join は文字列を構築するためのシンプルだが強力な方法です。
 */

fn main() {
    let helloworld = ["hello", " ", "world", "!"].concat();
    println!("{}", helloworld);
    let abc = ["a", "b", "c"].join(",");
    println!("{}", abc);
}
