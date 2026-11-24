use task_922::*;

#[test]
fn public_examples() {
    assert_eq!(
        shortest_field_with_first_tie_slash("path/1/12", |s| s.chars().count() + 1),
        2
    );
    assert_eq!(
        shortest_field_with_first_tie_slash("path/123", |s| s.len() * 2),
        6
    );
}
