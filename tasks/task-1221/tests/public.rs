use task_1221::*;

#[test]
fn public_examples() {
    assert_eq!(delimited_fields_trace(&[]), vec![]);
    assert_eq!(delimited_fields_trace(&[4]), vec![1u32]);
}
