use task_723::*;

#[test]
fn public_examples() {
    assert_eq!(
        shortest_field_with_first_tie_slash(Some("path/1/12")),
        Some(1)
    );
    assert_eq!(
        shortest_field_with_first_tie_slash(Some("path/123")),
        Some(123)
    );
}
