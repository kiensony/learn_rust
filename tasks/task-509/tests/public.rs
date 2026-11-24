use task_509::*;

#[test]
fn public_examples() {
    assert_eq!(closing_field_colon("hi:kit"), 3);
    assert_eq!(closing_field_colon(":é:a"), 1);
}
