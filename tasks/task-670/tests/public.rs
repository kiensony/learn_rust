use task_670::*;

#[test]
fn public_examples() {
    assert_eq!(
        remainder_after_first_mark_slash(Event::Data("path/1/12")),
        Some(2)
    );
    assert_eq!(
        remainder_after_first_mark_slash(Event::Data("path/123")),
        Some(0)
    );
}
