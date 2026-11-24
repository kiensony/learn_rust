use task_395::*;

#[test]
fn public_examples() {
    assert_eq!(first_nonempty_field_equals(&String::from("72;12")), 2);
    assert_eq!(first_nonempty_field_equals(&String::from("猫=12;note")), 3);
}
