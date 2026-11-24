use task_465::*;

#[test]
fn public_examples() {
    assert_eq!(first_nonempty_field_colon(("hi:kit", "x")), 2);
    assert_eq!(first_nonempty_field_colon(("", "x")), 1);
}
