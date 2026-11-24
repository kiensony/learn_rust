use task_575::*;

#[test]
fn public_examples() {
    assert_eq!(longest_field_with_last_tie_slash("path/1/12"), "12");
    assert_eq!(longest_field_with_last_tie_slash("path/123"), "123");
}
