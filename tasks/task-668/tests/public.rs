use task_668::*;

#[test]
fn public_examples() {
    assert_eq!(
        closing_marks_removed_slash(Event::Data("path/1/12")),
        Some(4)
    );
    assert_eq!(
        closing_marks_removed_slash(Event::Data("path/123")),
        Some(3)
    );
}
