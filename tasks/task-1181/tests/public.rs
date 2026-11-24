use task_1181::*;

#[test]
fn public_examples() {
    assert_eq!(leading_field_equals("72;12", "backup"), "72");
    assert_eq!(leading_field_equals("猫=12;note", "backup"), "猫");
}
