use task_1081::*;

#[test]
fn public_examples() {
    assert_eq!(leading_field_equals(&"72;12"), 2);
    assert_eq!(leading_field_equals(&String::from("猫=12;note")), 3);
}
