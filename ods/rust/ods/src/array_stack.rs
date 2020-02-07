pub struct Array {
    pub a: Vec<u32>,
    pub n: u32,
}

impl Array {
    pub fn new() -> Array {
        let mut vec = Vec::new();
        Array { a: vec, n: 0 }
    }

    pub fn size(&self) -> u32 {
        self.n
    }

    pub fn get(&self, i: usize) -> u32 {
        self.a[i]
    }

    pub fn set(&mut self, i: usize, x: u32) -> u32 {
        let y = self.a[i];
        self.a[i] = x;
        y
    }
}
