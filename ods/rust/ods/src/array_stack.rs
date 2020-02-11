pub struct ArrayStack {
    pub a: [u32; 1],
    pub n: usize,
}

impl ArrayStack {
    pub fn new() -> ArrayStack {
        let ary: [u32; 1] = [0];
        ArrayStack { a: ary, n: 0 }
    }

    pub fn size(&self) -> usize {
        self.n
    }

    pub fn get(&mut self, i: usize) -> &u32 {
        &self.a[i]
    }

    pub fn set(&mut self, i: usize, x: u32) -> u32 {
        let y: u32 = self.a[i].clone();
        self.a[i] = x;
        y
    }

    pub fn add(&self, i: usize, x: u32) {
        if self.n + 1 > self.a.len() {
            print!("resize!!!");
        }
    }

    pub fn remove(&self, i: usize) {}

    pub fn resize(&self) {}
}
