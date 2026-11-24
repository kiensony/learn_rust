use task_1185::*;

#[test]
fn public_examples() {
    assert_eq!(marks_removed_at_both_ends_equals("72;12", "backup"), "72");
    assert_eq!(
        marks_removed_at_both_ends_equals("猫=12;note", "backup"),
        "猫=12"
    );
}
