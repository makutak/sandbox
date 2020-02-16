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
                for j in (i..self.n).rev() {
                    self.a[j + 1] = self.a[j];
                }
                self.a[i] = x;
                self.n += 1;
            }
        }
    }

    pub fn remove(&mut self, i: usize) -> u32 {
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

    pub fn resize(&mut self) {
        let mut b = vec![0; max(1, 2 * self.n)];
        for i in 0..self.n {
            b[i] = self.a[i];
        }
        self.a = b;
    }
}

#[test]
fn new_array_stack_n_is_zero() {
    let ary = ArrayStack::new();
    assert_eq!(ary.n, 0);
}

#[test]
fn new_array_stack_a_length_is_zero() {
    let ary = ArrayStack::new();
    assert_eq!(ary.a.len(), 0);
}

#[test]
fn array_stack_size_is_variable() {
    let mut ary = ArrayStack::new();
    assert_eq!(ary.size(), 0);
    ary.add(0, 1);
    assert_eq!(ary.size(), 1);
    ary.add(0, 2);
    assert_eq!(ary.size(), 2);
    ary.add(0, 3);
    assert_eq!(ary.size(), 3);
}

#[test]
fn array_stack_can_get_selected_value() {
    let mut ary = ArrayStack::new();
    ary.add(0, 1);
    ary.add(0, 2);
    ary.add(0, 3);
    assert_eq!(ary.get(0), 3);
    assert_eq!(ary.get(1), 2);
    assert_eq!(ary.get(2), 1);
}

#[test]
fn array_stack_can_set_selected_value() {
    let mut ary = ArrayStack::new();
    ary.add(0, 1);
    ary.add(0, 2);
    ary.add(0, 3);
    assert_eq!(ary.a, [3, 2, 1, 0]);
    ary.set(1, 4);
    assert_eq!(ary.a, [3, 4, 1, 0]);
}

#[test]
fn array_stack_can_be_added_value() {
    let mut ary = ArrayStack::new();
    ary.add(0, 10);
    let mut expected = vec![10];
    assert_eq!(ary.a, expected);
    ary.add(0, 20);
    expected = vec![20, 10];
    assert_eq!(ary.a, expected);
    ary.add(1, 30);
    expected = vec![20, 30, 10, 0];
    assert_eq!(ary.a, expected);
}

#[test]
fn array_stack_can_be_removed_value() {
    let mut ary = ArrayStack::new();
    ary.add(0, 1);
    ary.add(0, 2);
    ary.add(0, 3);
    ary.add(0, 4);
    ary.add(0, 5);
    let expected = [5, 4, 2, 1, 0, 0, 0, 0];
    ary.remove(2);
    assert_eq!(ary.a, expected);
}
