/*
 * 変数とは異なり、定数は明示的な型指定が必要です。
 * 定数名には大文字のスネークケース SCREAMING_SNAKE_CASE を使用します。
 */
const PI: f32 = 3.141519;

fn main() {
    println!(
        "ゼロからアップル {} を作るには、まず宇宙を想像する必要があります。",
        PI
    );

    println!(
        "If you wish to make an apple {} from scratch, you must first invent the universe",
        PI
    );

    println!("カール・セーガンの言葉だそうです。");
}
