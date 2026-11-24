use task_692::*;

#[test]
fn public_examples() {
    assert_eq!(first_nonempty_field_equals(Event::Data("72;12")), Some(2));
    assert_eq!(
        first_nonempty_field_equals(Event::Data("猫=12;note")),
        Some(3)
    );
}
