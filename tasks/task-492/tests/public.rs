use task_492::*;

#[test]
fn public_examples() {
    assert_eq!(marks_removed_at_both_ends_equals(("72;12", "x")), 2);
    assert_eq!(marks_removed_at_both_ends_equals(("猫=12;note", "z=w")), 6);
}
