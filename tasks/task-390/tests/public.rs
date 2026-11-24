use task_390::*;

#[test]
fn public_examples() {
    assert_eq!(leading_marks_removed_equals(&String::from("72;12")), 2);
    assert_eq!(leading_marks_removed_equals(&String::from("猫=12;note")), 6);
}
