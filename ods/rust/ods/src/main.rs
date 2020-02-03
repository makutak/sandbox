#[derive(Debug)]
struct Array {
    a: Vec<u32>,
    n: u32,
}

impl Array {
    pub fn new() -> Array {
        let mut vec = Vec::new();
        Array { a: vec, n: 0 }
    }
}

fn main() {
    println!("Hello, world!");
    let ary = Array::new();
    println!("Array: {:?}", ary);
}
