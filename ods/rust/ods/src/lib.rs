pub mod array_stack;

#[cfg(test)]
mod tests {
    use super::*;
    use array_stack::*;

    #[test]
    fn new_array_stack_n_is_zero() {
        let ary = ArrayStack::new();
        assert_eq!(ary.n, 0);
    }

    /*
    #[test]
    fn new_array_stack_a_is_empty() {
        let ary = ArrayStack::new();
        assert_eq!(ary.n, 0);
    }

    #[test]
    fn new_array_stack_size_is_zero() {
        let ary = ArrayStack::new();
        assert_eq!(ary.size(), 0);
    }

    #[test]
    fn array_stack_can_be_added_value() {
        let mut ary = ArrayStack::new();
        //ary.add(0, 1);
        let aaa = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        println!("aaa[0..2]:{:?}", &aaa[0..2]);
    }
    */
}
