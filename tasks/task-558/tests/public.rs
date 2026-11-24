use task_558::*;

#[test]
fn public_examples() {
    assert_eq!(leading_field_colon("hi:kit"), "hi");
    assert_eq!(leading_field_colon(":é:a"), "");
}
