use task_669::*;

#[test]
fn public_examples() {
    assert_eq!(
        marks_removed_at_both_ends_slash(Event::Data("path/1/12")),
        Some(4)
    );
    assert_eq!(
        marks_removed_at_both_ends_slash(Event::Data("path/123")),
        Some(3)
    );
}
