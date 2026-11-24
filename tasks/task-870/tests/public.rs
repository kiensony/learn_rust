use task_870::*;

#[test]
fn public_examples() {
    assert_eq!(first_nonempty_field_slash("path/1/12"), 3);
    assert_eq!(first_nonempty_field_slash("path/123"), 5);
}
