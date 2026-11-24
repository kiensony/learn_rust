use task_741::*;

#[test]
fn public_examples() {
    assert_eq!(first_nonempty_field_equals(Some("72;12")), Some(72));
    assert_eq!(first_nonempty_field_equals(Some("猫=12;note")), None);
}
