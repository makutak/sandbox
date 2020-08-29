#[derive(Debug)]
enum Item {
    Inventory(String),
    // None は項目がないことを表す
    None,
}

struct BagOfHolding {
    item: Item,
}

fn main() {
    let bag_a = BagOfHolding {
        item: Item::Inventory(String::from("foo")),
    };
    let bag_none = BagOfHolding { item: Item::None };

    println!("bag_a: {:?}", bag_a.item);
    println!("bag_none: {:?}", bag_none.item);
}
