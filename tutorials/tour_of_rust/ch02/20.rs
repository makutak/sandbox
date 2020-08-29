/*
 * if、match、関数、ブロックは、単一の方法で値を返すことができる。
 * if、match、関数、ブロックの最後が ";" のない式であれば、戻り値として使用される。
 * その戻り値は新しい変数に入れることができる。
 */

fn example() -> i32 {
    let x = 42;

    // Rust の三項式
    let v = if x < 42 { -1 } else { 1 };
    println!("v: {}", v);

    let food = "ハンバーガー";
    let result = match food {
        "ホットドッグ" => "ホットドッグです",
        //  単一の四季で値を返す場合、中括弧は省略可能
        _ => "ホットドッグではありません",
    };
    println!("食品の識別: {}", result);

    let v = {
        // ブロックのスコープは関数のスコープから分離されている
        let a = 1;
        let b = 2;
        a + b
    };
    println!("ブロックより: {}", v);

    // Rust で関数の最後から値を返す慣用的な方法
    v + 4
}

fn main() {
    println!("関数より: {}", example());
}
