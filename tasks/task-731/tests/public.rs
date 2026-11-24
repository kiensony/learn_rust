use task_731::*;

#[test]
fn public_examples() {
    assert_eq!(first_nonempty_field_pipe(Some("first\n123")), Some(123));
    assert_eq!(first_nonempty_field_pipe(Some("header\n猫|123")), None);
}
