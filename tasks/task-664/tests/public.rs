use task_664::*;

#[test]
fn public_examples() {
    assert_eq!(
        shortest_field_with_first_tie_colon(Event::Data("hi:kit")),
        Some(2)
    );
    assert_eq!(
        shortest_field_with_first_tie_colon(Event::Data(":é:a")),
        Some(0)
    );
}
