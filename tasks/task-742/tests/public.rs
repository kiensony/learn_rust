use task_742::*;

#[test]
fn public_examples() {
    assert_eq!(longest_field_with_last_tie_equals(Some("72;12")), Some(72));
    assert_eq!(longest_field_with_last_tie_equals(Some("猫=12;note")), None);
}
