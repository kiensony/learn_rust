use task_724::*;

#[test]
fn public_examples() {
    assert_eq!(leading_field_pipe(Some("first\n123")), Some(123));
    assert_eq!(leading_field_pipe(Some("header\n猫|123")), None);
}
