use task_860::*;

#[test]
fn public_examples() {
    assert_eq!(first_nonempty_field_colon("hi:kit"), 4);
    assert_eq!(first_nonempty_field_colon(""), 2);
}
