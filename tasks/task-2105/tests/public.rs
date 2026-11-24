use task_2105::*;

#[test]
fn public_examples() {
    assert_eq!(sorted_keys_changes(&[]), vec![]);
    assert_eq!(sorted_keys_changes(&[4]), vec![4u32]);
}
