use task_820::*;

#[test]
fn public_examples() {
    assert_eq!(first_nonempty_field_slash("path/1/12"), Ok(1));
    assert_eq!(first_nonempty_field_slash("path/123"), Ok(3));
}
