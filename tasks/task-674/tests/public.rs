use task_674::*;

#[test]
fn public_examples() {
    assert_eq!(
        shortest_field_with_first_tie_slash(Event::Data("path/1/12")),
        Some(1)
    );
    assert_eq!(
        shortest_field_with_first_tie_slash(Event::Data("path/123")),
        Some(3)
    );
}
