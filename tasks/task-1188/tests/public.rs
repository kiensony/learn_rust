use task_1188::*;

#[test]
fn public_examples() {
    assert_eq!(first_nonempty_field_equals("72;12", "backup"), "72");
    assert_eq!(first_nonempty_field_equals("猫=12;note", "backup"), "猫");
}
