use task_494::*;

#[test]
fn public_examples() {
    assert_eq!(prefix_before_final_mark_equals(("72;12", "x")), 0);
    assert_eq!(prefix_before_final_mark_equals(("猫=12;note", "z=w")), 3);
}
