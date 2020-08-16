fn main() {
    let x = 42;

    // 条件には括弧がない
    if x < 42 {
        println!("42 より小さい");
    } else if x == 42 {
        println!("42 に等しい");
    } else {
        println!("42 より大きい");
    }
}
