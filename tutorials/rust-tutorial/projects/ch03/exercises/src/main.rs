fn main() {
    assert_eq!(0, fahrenheit_to_centigrade(32));
    assert_eq!(100, fahrenheit_to_centigrade(212))
}

//華氏から摂氏への変換
fn fahrenheit_to_centigrade(val: i32) -> i32 {
    (val - 32) * 5 / 9
}
