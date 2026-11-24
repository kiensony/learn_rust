use task_341::*;

#[test]
fn public_examples() {
    assert_eq!(closing_marks_removed_equals(String::from("72;12")), "72");
    assert_eq!(
        closing_marks_removed_equals(String::from("猫=12;note")),
        "猫=12"
    );
}
