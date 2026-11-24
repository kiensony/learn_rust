use task_885::*;

#[test]
fn public_examples() {
    assert_eq!(leading_marks_removed_equals("72;12"), 4);
    assert_eq!(leading_marks_removed_equals("猫=12;note"), 8);
}
