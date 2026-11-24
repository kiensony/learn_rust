use task_721::*;

#[test]
fn public_examples() {
    assert_eq!(first_nonempty_field_slash(Some("path/1/12")), Some(1));
    assert_eq!(first_nonempty_field_slash(Some("path/123")), Some(123));
}
