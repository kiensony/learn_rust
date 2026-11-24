use task_1133::*;

#[test]
fn public_examples() {
    assert_eq!(leading_marks_removed_equals(&"72;12"), 2);
    assert_eq!(leading_marks_removed_equals(&String::from("猫=12;note")), 6);
}
