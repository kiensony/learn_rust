use task_693::*;

#[test]
fn public_examples() {
    assert_eq!(
        longest_field_with_last_tie_equals(Event::Data("72;12")),
        Some(2)
    );
    assert_eq!(
        longest_field_with_last_tie_equals(Event::Data("猫=12;note")),
        Some(3)
    );
}
