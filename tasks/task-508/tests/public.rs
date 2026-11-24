use task_508::*;

#[test]
fn public_examples() {
    assert_eq!(leading_field_colon("hi:kit"), 2);
    assert_eq!(leading_field_colon(":é:a"), 0);
}
