use task_543::*;

#[test]
fn public_examples() {
    assert_eq!(remainder_after_first_mark_equals("72;12"), 0);
    assert_eq!(remainder_after_first_mark_equals("猫=12;note"), 2);
}
