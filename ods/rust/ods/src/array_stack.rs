pub struct ArrayStack {
    pub a: [u8; 1],
    pub n: u32,
}

impl ArrayStack {
    pub fn new() -> ArrayStack {
        let ary: [u8; 1] = [0];
        ArrayStack { a: ary, n: 0 }
    }

    pub fn size(&self) -> u32 {
        self.n
    }

    /*
    pub fn get(&self, i: usize) -> u32 {
        self.a[i]
    }

    pub fn set(&mut self, i: usize, x: u32) -> u32 {
        let y = self.a[i];
        self.a[i] = x;
        y
    }

    pub fn add(&mut self, i: usize, x: u32) {
        print!("this is add function!!");
    }
    */
}
