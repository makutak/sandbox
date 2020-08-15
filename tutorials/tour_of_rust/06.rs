fn main() {
    let a = 13u8;
    let b = 7u32;
    //let c = a + b; u8 + u32 はエラーになる
    // as で型の変換ができる
    let c = a as u32 + b;
    println!("{}", c);

    let t = true;
    println!("{} {}", t, t as u8);
}
