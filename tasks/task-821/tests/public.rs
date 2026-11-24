use task_821::*;

#[test]
fn public_examples() {
    assert_eq!(longest_field_with_last_tie_slash("path/1/12"), Ok(2));
    assert_eq!(longest_field_with_last_tie_slash("path/123"), Ok(3));
}
