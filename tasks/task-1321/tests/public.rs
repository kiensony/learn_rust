use task_1321::*;

#[test]
fn public_examples() {
    assert_eq!(flag_fields_trace(""), vec![]);
    assert_eq!(flag_fields_trace("1"), vec![1u32]);
}
