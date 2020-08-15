fn swap(x: i32, y: i32) -> (i32, i32) {
    return (y, x);
}

fn main() {
    // 戻り値をタプルで返す
    let result = swap(123, 321);
    println!("{} {}", result.0, result.1);

    // タプルを2つの変数に分解
    let (a, b) = swap(result.0, result.1);
    println!("{} {}", a, b);
}
