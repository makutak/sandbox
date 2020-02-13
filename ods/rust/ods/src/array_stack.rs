use std::cmp::max;

pub struct ArrayStack {
    pub a: Vec<u32>,
    pub n: usize,
}

// FIXME: 値渡し、ポインタ渡し、参照渡し 説明できる？？？
impl ArrayStack {
    pub fn new() -> ArrayStack {
        let ary = Vec::new();
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
    pub fn add(&mut self, i: usize, x: u32) {
        if self.n + 1 > self.a.len() {
            self.resize();
        }

        match i {
            idx if idx < usize::min_value() || idx > self.n => println!("IndexError!!"),
            _ => {
                let mut tmp = vec![0; self.n + 1];
                for j in i..self.n {
                    tmp[j + i] = self.a[j]
                }
                tmp[i] = x;
                self.a = tmp;
                self.n += 1;
            }
        }
    }

    pub fn remove(&self, i: usize) {}

    pub fn resize(&mut self) {
        let mut b = vec![0; max(1, 2 * self.n)];
        for i in 0..self.n {
            b[i] = self.a[i];
        }
        self.a = b;
    }
}

/*
   def add(self, i, x):
       if i < 0 or i > self.n: raise IndexError()
       if self.n == len(self.a): self._resize()
       self.a[i+1:self.n+1] = self.a[i:self.n]
       self.a[i] = x
       self.n += 1
*/
