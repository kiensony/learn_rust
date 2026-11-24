use task_475::*;

#[test]
fn public_examples() {
    assert_eq!(first_nonempty_field_slash(("path/1/12", "x")), 1);
    assert_eq!(first_nonempty_field_slash(("path/123", "z/w")), 3);
}
