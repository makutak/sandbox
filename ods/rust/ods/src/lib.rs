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
        assert_eq!(ary.a.len(), 0);
    }

    #[test]
    fn new_array_stack_size_is_zero() {
        let ary = ArrayStack::new();
        assert_eq!(ary.size(), 0);
    }

    #[test]
    fn array_stack_can_be_added_value() {}
    */
}
