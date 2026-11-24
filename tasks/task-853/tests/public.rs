use task_853::*;

#[test]
fn public_examples() {
    assert_eq!(leading_field_colon("hi:kit"), 4);
    assert_eq!(leading_field_colon(":é:a"), 2);
}
