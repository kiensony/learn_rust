use task_1230::*;

#[test]
fn public_examples() {
    assert_eq!(delimited_fields_reverse(&[]), vec![]);
    assert_eq!(delimited_fields_reverse(&[4]), vec![1u32]);
}
