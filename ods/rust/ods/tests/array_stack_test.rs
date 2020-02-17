use ods_rust::array_stack::*;

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

#[test]
#[should_panic]
fn get_out_of_range_array() {
    let mut ary = ArrayStack::new();
    ary.add(0, 1);
    ary.get(2);
}

#[test]
#[should_panic]
fn set_out_of_range_array() {
    let mut ary = ArrayStack::new();
    ary.add(0, 1);
    ary.set(2, 2);
}

#[test]
#[should_panic]
fn add_out_of_range_array() {
    let mut ary = ArrayStack::new();
    ary.add(1, 1);
}

#[test]
#[should_panic]
fn remove_out_of_range_array() {
    let mut ary = ArrayStack::new();
    ary.add(1, 1);
    ary.remove(100);
}
