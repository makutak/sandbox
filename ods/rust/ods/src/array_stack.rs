use std::cmp::max;

pub struct ArrayStack {
    pub a: Vec<u32>,
    pub n: usize,
}

impl ArrayStack {
    pub fn new() -> ArrayStack {
        let ary = vec![];
        ArrayStack { a: ary, n: 0 }
    }

    pub fn size(&self) -> usize {
        self.n
    }

    pub fn get(&mut self, i: usize) -> u32 {
        self.a[i]
    }

    pub fn set(&mut self, i: usize, x: u32) -> u32 {
        let y: u32 = self.a[i].clone();
        self.a[i] = x;
        y
    }

    pub fn add(&mut self, i: usize, x: u32) {
        if self.n + 1 > self.a.len() {
            self.resize();
        }
        match i {
            idx if idx < usize::min_value() || idx > self.n => panic!("IndexError!!"),
            _ => {
                println!("#############################################");
                println!("before self.a: {:?}", self.a);
                for j in (i..self.n).rev() {
                    println!("j: {}", j);
                    println!("self.a[j]: {}", self.a[j]);
                    self.a[j + 1] = self.a[j];
                }
                self.a[i] = x;
                println!("after self.a: {:?}", self.a);
                println!("#############################################");
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
        println!("resize: a: {:?}, b: {:?}", self.a, b);
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
