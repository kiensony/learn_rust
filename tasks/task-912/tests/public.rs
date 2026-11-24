use task_912::*;

#[test]
fn public_examples() {
    assert_eq!(
        shortest_field_with_first_tie_colon("hi:kit", |s| s.chars().count() + 1),
        3
    );
    assert_eq!(
        shortest_field_with_first_tie_colon(":é:a", |s| s.len() * 2),
        0
    );
}
