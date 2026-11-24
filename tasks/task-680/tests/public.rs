use task_680::*;

#[test]
fn public_examples() {
    assert_eq!(
        remainder_after_first_mark_pipe(Event::Data("first\n123")),
        Some(0)
    );
    assert_eq!(
        remainder_after_first_mark_pipe(Event::Data("header\n猫|123")),
        Some(3)
    );
}
