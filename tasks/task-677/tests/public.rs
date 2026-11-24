use task_677::*;

#[test]
fn public_examples() {
    assert_eq!(
        leading_marks_removed_pipe(Event::Data("first\n123")),
        Some(3)
    );
    assert_eq!(
        leading_marks_removed_pipe(Event::Data("header\n猫|123")),
        Some(7)
    );
}
