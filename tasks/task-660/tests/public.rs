use task_660::*;

#[test]
fn public_examples() {
    assert_eq!(
        marks_removed_at_both_ends_colon(Event::Data("hi:kit")),
        Some(6)
    );
    assert_eq!(
        marks_removed_at_both_ends_colon(Event::Data(":é:a")),
        Some(4)
    );
}
