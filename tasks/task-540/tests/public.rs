use task_540::*;

#[test]
fn public_examples() {
    assert_eq!(leading_marks_removed_equals("72;12"), 2);
    assert_eq!(leading_marks_removed_equals("猫=12;note"), 4);
}
