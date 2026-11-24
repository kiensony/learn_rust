use task_1162::*;

#[test]
fn public_examples() {
    assert_eq!(closing_field_slash("path/1/12", "backup"), "12");
    assert_eq!(closing_field_slash("path/123", "backup"), "123");
}
