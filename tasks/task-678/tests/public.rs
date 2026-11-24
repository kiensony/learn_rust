use task_678::*;

#[test]
fn public_examples() {
    assert_eq!(
        closing_marks_removed_pipe(Event::Data("first\n123")),
        Some(3)
    );
    assert_eq!(
        closing_marks_removed_pipe(Event::Data("header\n猫|123")),
        Some(7)
    );
}
