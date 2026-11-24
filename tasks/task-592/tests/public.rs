use task_592::*;

#[test]
fn public_examples() {
    assert_eq!(remainder_after_first_mark_equals("72;12"), "");
    assert_eq!(remainder_after_first_mark_equals("猫=12;note"), "12");
}
