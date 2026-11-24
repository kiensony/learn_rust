use task_496::*;

#[test]
fn public_examples() {
    assert_eq!(longest_field_with_last_tie_equals(("72;12", "x")), 2);
    assert_eq!(longest_field_with_last_tie_equals(("猫=12;note", "z=w")), 3);
}
