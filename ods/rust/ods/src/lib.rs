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

    #[test]
    fn new_array_stack_a_length_is_zero() {
        let ary = ArrayStack::new();
        assert_eq!(ary.a.len(), 0);
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
}
