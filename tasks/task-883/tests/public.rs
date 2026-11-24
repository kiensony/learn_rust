use task_883::*;

#[test]
fn public_examples() {
    assert_eq!(leading_field_equals("72;12"), 4);
    assert_eq!(leading_field_equals("猫=12;note"), 5);
}
