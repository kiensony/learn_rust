use task_515::*;

#[test]
fn public_examples() {
    assert_eq!(first_nonempty_field_colon("hi:kit"), 2);
    assert_eq!(first_nonempty_field_colon(":é:a"), 1);
}
