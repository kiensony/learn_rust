use task_871::*;

#[test]
fn public_examples() {
    assert_eq!(longest_field_with_last_tie_slash("path/1/12"), 4);
    assert_eq!(longest_field_with_last_tie_slash("path/123"), 5);
}
