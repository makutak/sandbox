pub struct ArrayStack {
    pub a: [u32; 1],
    pub n: usize,
}

// FIXME: 値渡し、ポインタ渡し、参照渡し 説明できる？？？
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

    // Result使う？
    pub fn add(&mut self, index: usize, x: u32) {
        if self.n + 1 > self.a.len() {
            print!("resize!!!");
        }

        match index {
            i if i < 0 || i > self.n => println!("indexError!!"),
            _ => println!("safe!!"),
        }
    }

    pub fn remove(&self, i: usize) {}

    pub fn resize(&self) {}
}

/*
   def add(self, i, x):
       if i < 0 or i > self.n: raise IndexError()
       if self.n == len(self.a): self._resize()
       self.a[i+1:self.n+1] = self.a[i:self.n]
       self.a[i] = x
       self.n += 1
*/
