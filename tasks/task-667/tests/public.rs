use task_667::*;

#[test]
fn public_examples() {
    assert_eq!(
        leading_marks_removed_slash(Event::Data("path/1/12")),
        Some(4)
    );
    assert_eq!(
        leading_marks_removed_slash(Event::Data("path/123")),
        Some(3)
    );
}
