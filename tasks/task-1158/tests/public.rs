use task_1158::*;

#[test]
fn public_examples() {
    assert_eq!(first_nonempty_field_colon("hi:kit", "backup"), "hi");
    assert_eq!(first_nonempty_field_colon(":é:a", "backup"), "é");
}
