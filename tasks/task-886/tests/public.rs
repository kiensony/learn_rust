use task_886::*;

#[test]
fn public_examples() {
    assert_eq!(closing_marks_removed_equals("72;12"), 4);
    assert_eq!(closing_marks_removed_equals("猫=12;note"), 8);
}
