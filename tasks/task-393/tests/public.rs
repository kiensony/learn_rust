use task_393::*;

#[test]
fn public_examples() {
    assert_eq!(remainder_after_first_mark_equals(&String::from("72;12")), 0);
    assert_eq!(
        remainder_after_first_mark_equals(&String::from("猫=12;note")),
        2
    );
}
