use task_1307::*;

#[test]
fn public_examples() {
    assert_eq!(decimal_fields_distinct(""), vec![]);
    assert_eq!(decimal_fields_distinct("1"), vec![1u32]);
}
