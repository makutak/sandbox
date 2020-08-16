// Rust ではタプルのような構造体も利用できる
struct Location(i32, i32);

fn main() {
    // これもスタックに入れられる構造体
    let loc = Location(42, 32);
    println!("{}, {}", loc.0, loc.1);
}
