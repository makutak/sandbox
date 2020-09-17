/*
Rustではutf-8バイトのシーケンスを char 型の文字のベクトルとして取得する方法が提供されている。
char は常に 4 バイトの長さです。(これによって個々の文字を効率的に検索できるようになっている。
 */
fn main() {
    // 文字を char のベクタとして集める
    let chars = "hi 🦀".chars().collect::<Vec<char>>();
    println!("{}", chars.len());
    // chars は 4バイトなので、 u32 に変換することができる
    println!("{}", chars[3] as u32);
}
