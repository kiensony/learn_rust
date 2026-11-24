use task_538::*;

#[test]
fn public_examples() {
    assert_eq!(leading_field_equals("72;12"), 2);
    assert_eq!(leading_field_equals("猫=12;note"), 1);
}
