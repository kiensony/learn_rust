use task_689::*;

#[test]
fn public_examples() {
    assert_eq!(
        marks_removed_at_both_ends_equals(Event::Data("72;12")),
        Some(2)
    );
    assert_eq!(
        marks_removed_at_both_ends_equals(Event::Data("猫=12;note")),
        Some(6)
    );
}
