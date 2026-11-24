use task_339::*;

#[test]
fn public_examples() {
    assert_eq!(closing_field_equals(String::from("72;12")), "72");
    assert_eq!(closing_field_equals(String::from("猫=12;note")), "12");
}
