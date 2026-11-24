use task_345::*;

#[test]
fn public_examples() {
    assert_eq!(first_nonempty_field_equals(String::from("72;12")), "72");
    assert_eq!(
        first_nonempty_field_equals(String::from("猫=12;note")),
        "猫"
    );
}
