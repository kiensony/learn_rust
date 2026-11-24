use task_1151::*;

#[test]
fn public_examples() {
    assert_eq!(leading_field_colon("hi:kit", "backup"), "hi");
    assert_eq!(leading_field_colon(":é:a", "backup"), "backup");
}
