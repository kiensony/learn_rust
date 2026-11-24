use task_687::*;

#[test]
fn public_examples() {
    assert_eq!(leading_marks_removed_equals(Event::Data("72;12")), Some(2));
    assert_eq!(
        leading_marks_removed_equals(Event::Data("猫=12;note")),
        Some(6)
    );
}
