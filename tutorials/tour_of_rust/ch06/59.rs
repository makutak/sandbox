/*

文字列リテラルは常にUnicodeである。文字列リテラルの型は &'static str である。
* & はメモリ中の場所を参照していることを意味しており、&mut を欠いているのは、
  コンパイラがそれを修正することを認めていないということである。
* 'static は文字列データがプログラムの終了まで有効であるということを意味している。
  (文字列データは決して消えない。)
* str は常に有効なutf-8であるバイト列を指していることを意味している。

*/

fn main() {
    let a: &'static str = "こんにちは 🦀";
    println!("{}, length: {}", a, a.len());
}
