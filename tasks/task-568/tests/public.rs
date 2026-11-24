use task_568::*;

#[test]
fn public_examples() {
    assert_eq!(closing_field_slash("path/1/12"), "12");
    assert_eq!(closing_field_slash("path/123"), "123");
}
