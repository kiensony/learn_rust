use task_519::*;

#[test]
fn public_examples() {
    assert_eq!(closing_field_slash("path/1/12"), 2);
    assert_eq!(closing_field_slash("path/123"), 3);
}
