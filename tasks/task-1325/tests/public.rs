use task_1325::*;

#[test]
fn public_examples() {
    assert_eq!(flag_fields_changes(""), vec![]);
    assert_eq!(flag_fields_changes("1"), vec![1u32]);
}
