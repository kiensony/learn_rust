use task_489::*;

#[test]
fn public_examples() {
    assert_eq!(closing_field_equals(("72;12", "x")), 2);
    assert_eq!(closing_field_equals(("hi=kit", "x")), 3);
}
