use task_564::*;

#[test]
fn public_examples() {
    assert_eq!(first_nonempty_field_colon("hi:kit"), "hi");
    assert_eq!(first_nonempty_field_colon(":é:a"), "é");
}
