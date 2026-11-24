use task_589::*;

#[test]
fn public_examples() {
    assert_eq!(leading_marks_removed_equals("72;12"), "72");
    assert_eq!(leading_marks_removed_equals("猫=12;note"), "猫=12");
}
