pub mod array_stack;

#[cfg(test)]
mod tests {
    use super::*;
    use array_stack::*;

    #[test]
    fn new_array_stack_n_is_zero() {
        let ary = Array::new();
        assert_eq!(ary.n, 0);
    }

    #[test]
    fn new_array_stack_a_is_empty() {
        let ary = Array::new();
        assert_eq!(ary.a.len(), 0);
    }

}
