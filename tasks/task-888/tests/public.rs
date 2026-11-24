use task_888::*;

#[test]
fn public_examples() {
    assert_eq!(remainder_after_first_mark_equals("72;12"), 2);
    assert_eq!(remainder_after_first_mark_equals("猫=12;note"), 4);
}
