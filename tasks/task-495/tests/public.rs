use task_495::*;

#[test]
fn public_examples() {
    assert_eq!(first_nonempty_field_equals(("72;12", "x")), 2);
    assert_eq!(first_nonempty_field_equals(("猫=12;note", "z=w")), 3);
}
