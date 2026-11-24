use task_1168::*;

#[test]
fn public_examples() {
    assert_eq!(first_nonempty_field_slash("path/1/12", "backup"), "1");
    assert_eq!(first_nonempty_field_slash("path/123", "backup"), "123");
}
