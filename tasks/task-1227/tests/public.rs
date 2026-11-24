use task_1227::*;

#[test]
fn public_examples() {
    assert_eq!(delimited_fields_distinct(&[]), vec![]);
    assert_eq!(delimited_fields_distinct(&[4]), vec![1u32]);
}
