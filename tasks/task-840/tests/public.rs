use task_840::*;

#[test]
fn public_examples() {
    assert_eq!(first_nonempty_field_equals("72;12"), Ok(2));
    assert_eq!(first_nonempty_field_equals("猫=12;note"), Ok(3));
}
