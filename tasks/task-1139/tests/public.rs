use task_1139::*;

#[test]
fn public_examples() {
    assert_eq!(longest_field_with_last_tie_equals(&"72;12"), 2);
    assert_eq!(
        longest_field_with_last_tie_equals(&String::from("猫=12;note")),
        3
    );
}
