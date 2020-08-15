fn main() {
    // 配列のデータ型は [T;N] であり、T は要素の型、N はコンパイル時に決まる固定長である。
    let nums: [i32; 3] = [1, 2, 3];
    println!("{:?}", nums);
    println!("{}", nums[1]);
}
