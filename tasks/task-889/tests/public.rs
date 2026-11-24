use task_889::*;

#[test]
fn public_examples() {
    assert_eq!(prefix_before_final_mark_equals("72;12"), 2);
    assert_eq!(prefix_before_final_mark_equals("猫=12;note"), 5);
}
