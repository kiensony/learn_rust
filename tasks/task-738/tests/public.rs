use task_738::*;

#[test]
fn public_examples() {
    assert_eq!(marks_removed_at_both_ends_equals(Some("72;12")), Some(72));
    assert_eq!(marks_removed_at_both_ends_equals(Some("猫=12;note")), None);
}
