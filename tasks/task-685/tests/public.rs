use task_685::*;

#[test]
fn public_examples() {
    assert_eq!(leading_field_equals(Event::Data("72;12")), Some(2));
    assert_eq!(leading_field_equals(Event::Data("猫=12;note")), Some(3));
}
