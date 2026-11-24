use task_491::*;

#[test]
fn public_examples() {
    assert_eq!(closing_marks_removed_equals(("72;12", "x")), 2);
    assert_eq!(closing_marks_removed_equals(("猫=12;note", "z=w")), 6);
}
