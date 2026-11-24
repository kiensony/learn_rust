use task_890::*;

#[test]
fn public_examples() {
    assert_eq!(first_nonempty_field_equals("72;12"), 4);
    assert_eq!(first_nonempty_field_equals("猫=12;note"), 5);
}
