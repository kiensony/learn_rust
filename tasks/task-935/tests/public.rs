use task_935::*;

#[test]
fn public_examples() {
    assert_eq!(
        leading_marks_removed_equals("72;12", |s| s.chars().count() + 1),
        3
    );
    assert_eq!(
        leading_marks_removed_equals("猫=12;note", |s| s.len() * 2),
        12
    );
}
