use task_864::*;

#[test]
fn public_examples() {
    assert_eq!(closing_field_slash("path/1/12"), 4);
    assert_eq!(closing_field_slash("path/123"), 5);
}
