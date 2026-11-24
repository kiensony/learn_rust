use task_884::*;

#[test]
fn public_examples() {
    assert_eq!(closing_field_equals("72;12"), 4);
    assert_eq!(closing_field_equals("hi=kit"), 5);
}
