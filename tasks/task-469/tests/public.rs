use task_469::*;

#[test]
fn public_examples() {
    assert_eq!(closing_field_slash(("path/1/12", "x")), 2);
    assert_eq!(closing_field_slash(("path/123", "z/w")), 3);
}
