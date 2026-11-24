use task_725::*;

#[test]
fn public_examples() {
    assert_eq!(closing_field_pipe(Some("first\n123")), Some(123));
    assert_eq!(closing_field_pipe(Some("hi|kit")), None);
}
