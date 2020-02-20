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
                println!("####################");
                println!("self.n: {}", self.n);
                let mut vec = vec![0; max(1, 2 * self.n)];
                println!("initial vec: {:?}, vec.length is: {}", vec, vec.len());
                println!("self.a: {:?}, self.a.length is: {}", self.a, self.a.len());
                println!("i: {}, x: {}", i, x);
                vec[..self.n].clone_from_slice(&self.a[i..self.n]);
                println!("cloned vec: {:?}, vec.length is: {}", vec, vec.len());

                println!("vec: {:?}", vec);
                //println!("&vec[..vec.len()]: {:?}", &vec[i..]);
                println!("self.a[i+1..self.n+1]: {:?}", &self.a[i + i..self.n + 1]);
                println!("self.a: {:?}", &self.a);
                println!("vec[0..self.n]: {:?}", &vec[0..self.n]);
                println!("vec[..self.n]: {:?}", &vec[..self.n]);
                println!("vec[..self.a.len()]: {:?}", &vec[..self.a.len()]);

                let len = self.a.len();
                println!("vec[..len]: {:?}", &vec[..len]);
                println!("vec[i..self.n]: {:?}", &vec[i..self.n]);
                println!("a[i + 1..]: {:?}", &self.a[i + 1..]);

                self.a[i + 1..].clone_from_slice(&vec[i..len - 1]);
                self.a[i] = x;
                println!("####################");
                self.n += 1;
            }
        }
    }

    pub fn remove(&mut self, i: usize) -> u32 {
        match i {
            idx if idx < usize::min_value() || idx > self.n => panic!("IndexError!!"),
            _ => {
                let x = self.a[i];
                for j in i..self.n {
                    self.a[j] = self.a[j + 1];
                }
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
        println!("initial resize b: {:?}", b);
        b[..self.a.len()].clone_from_slice(&self.a);
        println!("resize: {:?}", b);
        self.a = b;
    }
}
