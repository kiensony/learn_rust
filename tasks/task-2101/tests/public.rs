use task_2101::*;

#[test]
fn public_examples() {
    assert_eq!(sorted_keys_trace(&[]), vec![]);
    assert_eq!(sorted_keys_trace(&[4]), vec![4u32]);
}
