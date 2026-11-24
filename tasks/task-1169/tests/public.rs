use task_1169::*;

#[test]
fn public_examples() {
    assert_eq!(
        longest_field_with_last_tie_slash("path/1/12", "backup"),
        "12"
    );
    assert_eq!(
        longest_field_with_last_tie_slash("path/123", "backup"),
        "123"
    );
}
