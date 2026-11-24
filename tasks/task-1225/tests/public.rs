use task_1225::*;

#[test]
fn public_examples() {
    assert_eq!(delimited_fields_changes(&[]), vec![]);
    assert_eq!(delimited_fields_changes(&[4]), vec![1u32]);
}
