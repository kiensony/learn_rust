use task_739::*;

#[test]
fn public_examples() {
    assert_eq!(remainder_after_first_mark_equals(Some("72;12")), None);
    assert_eq!(
        remainder_after_first_mark_equals(Some("猫=12;note")),
        Some(12)
    );
}
