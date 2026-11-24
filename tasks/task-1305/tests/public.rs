use task_1305::*;

#[test]
fn public_examples() {
    assert_eq!(decimal_fields_changes(""), vec![]);
    assert_eq!(decimal_fields_changes("1"), vec![1u32]);
}
