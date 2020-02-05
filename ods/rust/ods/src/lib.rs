pub mod array_stack;

#[cfg(test)]
mod tests {
    use super::*;
    use array_stack::*;

    #[test]
    fn new_array_stack_size_is_zero() {
        let ary = Array::new();
        assert_eq!(ary.size(), 0);
    }
}
