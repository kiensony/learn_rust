use task_1152::*;

#[test]
fn public_examples() {
    assert_eq!(closing_field_colon("hi:kit", "backup"), "kit");
    assert_eq!(closing_field_colon(":é:a", "backup"), "a");
}
