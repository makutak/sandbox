pub struct Array {
    a: Vec<u32>,
    n: u32,
}

impl Array {
    pub fn new() -> Array {
        let mut vec = Vec::new();
        Array { a: vec, n: 0 }
    }

    pub fn size(&self) -> u32 {
        self.n
    }
}
