use task_1087::*;

#[test]
fn public_examples() {
    assert_eq!(prefix_before_final_mark_equals(&"72;12"), 0);
    assert_eq!(
        prefix_before_final_mark_equals(&String::from("猫=12;note")),
        3
    );
}
