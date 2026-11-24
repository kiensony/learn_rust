use task_694::*;

#[test]
fn public_examples() {
    assert_eq!(
        shortest_field_with_first_tie_equals(Event::Data("72;12")),
        Some(2)
    );
    assert_eq!(
        shortest_field_with_first_tie_equals(Event::Data("=é=a")),
        Some(0)
    );
}
