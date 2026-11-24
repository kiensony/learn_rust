use task_711::*;

#[test]
fn public_examples() {
    assert_eq!(first_nonempty_field_colon(Some("hi:kit")), None);
    assert_eq!(first_nonempty_field_colon(Some("3:72")), Some(3));
}
