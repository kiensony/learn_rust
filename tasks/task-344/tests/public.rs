use task_344::*;

#[test]
fn public_examples() {
    assert_eq!(prefix_before_final_mark_equals(String::from("72;12")), "");
    assert_eq!(
        prefix_before_final_mark_equals(String::from("猫=12;note")),
        "猫"
    );
}
