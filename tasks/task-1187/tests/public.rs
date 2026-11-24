use task_1187::*;

#[test]
fn public_examples() {
    assert_eq!(prefix_before_final_mark_equals("72;12", "backup"), "backup");
    assert_eq!(
        prefix_before_final_mark_equals("猫=12;note", "backup"),
        "猫"
    );
}
