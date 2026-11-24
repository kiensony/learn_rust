use task_686::*;

#[test]
fn public_examples() {
    assert_eq!(closing_field_equals(Event::Data("72;12")), Some(2));
    assert_eq!(closing_field_equals(Event::Data("hi=kit")), Some(3));
}
