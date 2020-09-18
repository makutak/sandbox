fn main() {
    let a = "hi 🦀";
    println!("{}", a.len());

    let first_word = &a[0..2];
    let second_word = &a[3..7];
    // let half_crab  &a[3..5]; は失敗する
    // Rustは無効な unicode 文字のwスライスを受け付けない。
    println!("{} {}", first_word, second_word);
}
