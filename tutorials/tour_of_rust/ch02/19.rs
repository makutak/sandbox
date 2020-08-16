fn main() {
    let mut x = 0;

    // loop は break で抜けて値を返すことができる
    let v = loop {
        x += 1;
        if x == 13 {
            // これが返り値になる
            break "13を発見！";
        }
    };
    println!("loopの戻り値: {}", v);
}
