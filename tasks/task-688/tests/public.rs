use task_688::*;

#[test]
fn public_examples() {
    assert_eq!(closing_marks_removed_equals(Event::Data("72;12")), Some(2));
    assert_eq!(
        closing_marks_removed_equals(Event::Data("猫=12;note")),
        Some(6)
    );
}
