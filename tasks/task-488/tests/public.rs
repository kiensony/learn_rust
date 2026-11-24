use task_488::*;

#[test]
fn public_examples() {
    assert_eq!(leading_field_equals(("72;12", "x")), 2);
    assert_eq!(leading_field_equals(("猫=12;note", "z=w")), 3);
}
