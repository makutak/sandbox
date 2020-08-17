/*
 * Option
 * Rust には `Option` と呼ばれるジェネリックな列挙型が組み込まれており、
 * `null` を使わずに null 許容な値を表現できる。
 */

// 部分的に定義された構造体型
struct BagOfHolding<T> {
    // パラメータ T を渡すことが可能
    item: Option<T>,
}

fn main() {
    // 注意: i32 が入るバッグに何も入っていない！
    // None からは方が決められないため、型を指定する必要があります。
    let i32_bag = BagOfHolding::<i32> { item: None };

    if i32_bag.item.is_none() {
        println!("バッグには何もない！");
    } else {
        println!("バッグには何かある！");
    }

    //let i32_bag = BagOfHolding::<i32> { item: Some(42) };
    let i32_bag = BagOfHolding::<i32> { item: None };

    if i32_bag.item.is_some() {
        println!("バッグには何かある！");
    } else {
        println!("バッグには何もない！");
    }

    // match は Option をエレガントに分解して、
    // すべてのケースが処理されることを保証できます！
    match i32_bag.item {
        Some(v) => println!("バッグに {} を発見！", v),
        None => println!("何も見つからなかった"),
    };
}
