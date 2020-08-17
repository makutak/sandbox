fn main() {
    // 型を明示的に指定
    let mut i32_vec = Vec::<i32>::new();
    i32_vec.push(1);
    i32_vec.push(2);
    i32_vec.push(3);

    for i in i32_vec.iter() {
        println!("{}", i);
    }

    // もっと賢く、型を自動的に推論
    let mut float_vec = Vec::new();
    float_vec.push(1.3);
    float_vec.push(2.3);
    float_vec.push(3.4);

    for i in float_vec.iter() {
        println!("{}", i);
    }

    // vec! はマクロ
    let string_vec = vec![String::from("Hello"), String::from("World!")];

    for word in string_vec.iter() {
        println!("{}", word);
    }
}

/*
* Vec は構造体ですが、内部的にはヒープ上の固定リストへの参照を含んでいる。
* ベクタはデフォルトの容量で始まる。
* 容量よりも多くの項目が追加された場合、
  ヒープ上により大きな容量の固定リストを生成して、データを再割り当てする。
*/
