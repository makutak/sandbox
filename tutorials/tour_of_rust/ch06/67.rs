/*
Stringはヒープメモリにutf-8バイトのシーケンスを持つ構造体である。
ヒープメモリ上にあるため、文字列リテラルではできない方法で拡張、変更などが可能。
Stringがドロップされるとき、そのヒープメモリもまたドロップされる。
 */
fn main() {
    let mut helloworld = String::from("hello");
    println!("helloworld: {}", helloworld);
    helloworld.push_str(" world");
    println!("helloworld: {}", helloworld);
    helloworld = helloworld + "!";
    println!("helloworld: {}", helloworld);
}
