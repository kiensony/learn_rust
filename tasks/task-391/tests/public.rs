use task_391::*;

#[test]
fn public_examples() {
    assert_eq!(closing_marks_removed_equals(&String::from("72;12")), 2);
    assert_eq!(closing_marks_removed_equals(&String::from("猫=12;note")), 6);
}
