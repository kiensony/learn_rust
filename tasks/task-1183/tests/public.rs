use task_1183::*;

#[test]
fn public_examples() {
    assert_eq!(leading_marks_removed_equals("72;12", "backup"), "72");
    assert_eq!(
        leading_marks_removed_equals("猫=12;note", "backup"),
        "猫=12"
    );
}
