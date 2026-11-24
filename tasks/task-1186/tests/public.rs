use task_1186::*;

#[test]
fn public_examples() {
    assert_eq!(
        remainder_after_first_mark_equals("72;12", "backup"),
        "backup"
    );
    assert_eq!(
        remainder_after_first_mark_equals("猫=12;note", "backup"),
        "12"
    );
}
