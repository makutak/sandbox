use std::cmp::max;

pub struct FastArrayStack {
    pub a: Vec<u32>,
    pub n: usize,
}

impl FastArrayStack {
    pub fn new() -> FastArrayStack {
        let ary = vec![];
        FastArrayStack { a: ary, n: 0 }
    }

    pub fn size(&self) -> usize {
        self.n
    }

    pub fn get(&mut self, i: usize) -> u32 {
        match i {
            idx if idx < usize::min_value() || idx > self.n => panic!("IndexError!!"),
            _ => self.a[i],
        }
    }

    pub fn set(&mut self, i: usize, x: u32) -> u32 {
        match i {
            idx if idx < usize::min_value() || idx > self.n => panic!("IndexError!!"),
            _ => {
                let y: u32 = self.a[i].clone();
                self.a[i] = x;
                y
            }
        }
    }

    pub fn add(&mut self, i: usize, x: u32) {
        if self.n + 1 > self.a.len() {
            self.resize();
        }
        match i {
            idx if idx < usize::min_value() || idx > self.n => panic!("IndexError!!"),
            _ => {
                let mut tmp = vec![0; max(1, 2 * self.n)];
                tmp[i..self.n].clone_from_slice(&self.a[i..self.n]);
                self.a[i + 1..self.n + 1].clone_from_slice(&tmp[i..self.n]);
                self.a[i] = x;
                self.n += 1;
            }
        }
    }

    pub fn remove(&mut self, i: usize) -> u32 {
        match i {
            idx if idx < usize::min_value() || idx > self.n => panic!("IndexError!!"),
            _ => {
                let x = self.a[i];
                let mut tmp = vec![0; max(1, 2 * self.n)];
                tmp[i + 1..self.n + 1].clone_from_slice(&self.a[i + 1..self.n + 1]);
                self.a[i..self.n].clone_from_slice(&tmp[i + 1..self.n + 1]);
                self.n -= 1;
                self.resize();
                if self.a.len() >= (3 * self.n) {
                    self.resize();
                }
                x
            }
        }
    }

    pub fn resize(&mut self) {
        let mut b = vec![0; max(1, 2 * self.n)];
        b[..self.a.len()].clone_from_slice(&self.a);
        self.a = b;
    }
}
