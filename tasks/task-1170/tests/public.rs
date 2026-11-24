use task_1170::*;

#[test]
fn public_examples() {
    assert_eq!(
        shortest_field_with_first_tie_slash("path/1/12", "backup"),
        "1"
    );
    assert_eq!(
        shortest_field_with_first_tie_slash("path/123", "backup"),
        "123"
    );
}
