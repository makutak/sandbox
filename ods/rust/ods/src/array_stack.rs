pub struct ArrayStack<T> {
    pub a: [T; 0],
    pub n: u32,
}

impl<T> ArrayStack<T> {
    pub fn new() -> ArrayStack<T> {
        let ary: [T; 0] = [];
        ArrayStack { a: ary, n: 0 }
    }

    pub fn size(&self) -> u32 {
        self.n
    }

    pub fn add(&self, i: usize, x: T) {}

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
