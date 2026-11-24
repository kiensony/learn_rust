use task_673::*;

#[test]
fn public_examples() {
    assert_eq!(
        longest_field_with_last_tie_slash(Event::Data("path/1/12")),
        Some(2)
    );
    assert_eq!(
        longest_field_with_last_tie_slash(Event::Data("path/123")),
        Some(3)
    );
}
