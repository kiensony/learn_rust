use task_338::*;

#[test]
fn public_examples() {
    assert_eq!(leading_field_equals(String::from("72;12")), "72");
    assert_eq!(leading_field_equals(String::from("猫=12;note")), "猫");
}
