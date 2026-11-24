use task_834::*;

#[test]
fn public_examples() {
    assert_eq!(closing_field_equals("72;12"), Ok(2));
    assert_eq!(closing_field_equals("hi=kit"), Ok(3));
}
