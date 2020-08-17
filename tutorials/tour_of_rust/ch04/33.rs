/*
 * ジェネリック型とは
 * ジェネリック型は `struct` や `enum` を部分的に定義することを可能にします。
 * コンパイル時にコードでの使用状況に基づいて完全に定義されたバージョンが生成されます。
 * Rust は通常、インスタンスを生成するコードから最終的な型を推論できます。
 * しかし補助が必要な場合は `::<T>` 演算子を使って明示的に指定できます。
 * この演算子は turbofish という名前で知られています。
 */

// 部分的に定義された構造体型
struct BagOfHolding<T> {
    item: T,
}

fn main() {
    // 注意: ジェネリック型を使用すると、型はコンパイル時に作成される。
    // ::<> (tubofish) 明示的に型を指定
    let i32_bag = BagOfHolding::<i32> { item: 42 };
    let bool_bag = BagOfHolding::<bool> { item: true };

    // ジェネリック型でも型推論可能
    let float_bag = BagOfHolding { item: 3.14 };

    // 注意: 実生活では手提げ袋をr手提げ袋に入れないように
    let bag_in_bag = BagOfHolding {
        item: BagOfHolding { item: "boom!" },
    };

    println!(
        "{} {} {} {}",
        i32_bag.item, bool_bag.item, float_bag.item, bag_in_bag.item.item
    );
}
