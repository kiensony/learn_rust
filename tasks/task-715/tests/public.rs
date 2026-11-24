use task_715::*;

#[test]
fn public_examples() {
    assert_eq!(closing_field_slash(Some("path/1/12")), Some(12));
    assert_eq!(closing_field_slash(Some("path/123")), Some(123));
}
