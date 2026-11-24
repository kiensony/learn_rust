use task_2107::*;

#[test]
fn public_examples() {
    assert_eq!(sorted_keys_distinct(&[]), vec![]);
    assert_eq!(sorted_keys_distinct(&[4]), vec![4u32]);
}
