use task_814::*;

#[test]
fn public_examples() {
    assert_eq!(closing_field_slash("path/1/12"), Ok(2));
    assert_eq!(closing_field_slash("path/123"), Ok(3));
}
