use task_679::*;

#[test]
fn public_examples() {
    assert_eq!(
        marks_removed_at_both_ends_pipe(Event::Data("first\n123")),
        Some(3)
    );
    assert_eq!(
        marks_removed_at_both_ends_pipe(Event::Data("header\n猫|123")),
        Some(7)
    );
}
