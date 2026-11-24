use task_2110::*;

#[test]
fn public_examples() {
    assert_eq!(sorted_keys_reverse(&[]), vec![]);
    assert_eq!(sorted_keys_reverse(&[4]), vec![4u32]);
}
