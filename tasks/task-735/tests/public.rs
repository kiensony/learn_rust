use task_735::*;

#[test]
fn public_examples() {
    assert_eq!(closing_field_equals(Some("72;12")), Some(72));
    assert_eq!(closing_field_equals(Some("猫=12;note")), Some(12));
}
