mod array_stack {
    struct Array {
        a: Vec<u32>,
        n: u32,
    }

    impl Array {
        fn new() -> Array {
            let mut vec = Vec::new();
            Array { a: vec, n: 0 }
        }

        fn size(&self) -> u32 {
            self.n
        }
    }
}
