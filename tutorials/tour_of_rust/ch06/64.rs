/*
非常に大きなテキストがある場合は、 include_str! というマクロを使って
ローカルファイルのテキストをプログラムの中でインクルードすると良い
 */

fn main() {
    let txt = include_str!("sample-text-file.txt");
    println!("{}", txt);
}
