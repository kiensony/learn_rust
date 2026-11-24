use task_837::*;

#[test]
fn public_examples() {
    assert_eq!(marks_removed_at_both_ends_equals("72;12"), Ok(2));
    assert_eq!(marks_removed_at_both_ends_equals("猫=12;note"), Ok(6));
}
