/*
スタティック変数はコンパイル時に作成され、プログラムの開始から終了まで存在するメモリリソースである。
これらの変数の型は明示的に指定しなければならない。

スタティックライフタイムは、プログラムの終了まで無期限に持続するメモリリソースである。
この定義では、スタティックライフタイムを持つリソースは実行時にも作成できることに注意。

スタティックライフタイムを持つリソースには、特別なライフタイム指定子 'static がある。
'static リソースは決してドロップすることはない。

スタティックライフタイムを持つリソースが参照を含む場合、それらはすべて'staticでないといけない。
(そうでなければ、参照はプログラムの終了前にドロップする可能性がある)
 */

static PI: f64 = 3.1415;

fn main() {
    // スタティック変数は関数スコープでも定義可能
    static mut SECRET: &'static str = "swordfish";

    // 文字列リテラルは 'static ライフタイム
    let msg: &'static str = "Hello World!";

    let p: &'static f64 = &PI;
    println!("{} {}", msg, p);

    // ルールを破ることはできるが、それを明示する必要がある
    unsafe {
        println!("SECRET: {}", SECRET);

        // 文字列リテラルは 'static なので SECRET に代入可能
        SECRET = "abracadabra";
        println!("SECRET: {}", SECRET);
    }
}