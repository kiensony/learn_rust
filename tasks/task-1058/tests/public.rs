use task_1058::*;

#[test]
fn public_examples() {
    assert_eq!(first_nonempty_field_colon(&"hi:kit"), 2);
    assert_eq!(first_nonempty_field_colon(&""), 0);
}
