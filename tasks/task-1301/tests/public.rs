use task_1301::*;

#[test]
fn public_examples() {
    assert_eq!(decimal_fields_trace(""), vec![]);
    assert_eq!(decimal_fields_trace("1"), vec![1u32]);
}
