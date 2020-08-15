fn main() {
    let x = 12; //デフォルトでは i32
    let a = 12u8; // u8として指定
    let b = 4.3; //デフォルトではf64
    let c = 4.3f32; //f32tとして指定
                    //数値型は、数値の最後に型を付加することで明示的に指定できます。
                    //（例: 13u32, 2u8）
    let bv = true;
    let t = (13, false);
    let sentence = "hello world";
    println!(
        "{} {} {} {} {} {} {} {}",
        x, a, b, c, bv, t.0, t.1, sentence
    );
}
