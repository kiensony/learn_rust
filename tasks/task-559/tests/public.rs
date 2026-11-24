use task_559::*;

#[test]
fn public_examples() {
    assert_eq!(closing_field_colon("hi:kit"), "kit");
    assert_eq!(closing_field_colon(":é:a"), "a");
}
