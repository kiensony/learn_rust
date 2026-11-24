use task_854::*;

#[test]
fn public_examples() {
    assert_eq!(closing_field_colon("hi:kit"), 5);
    assert_eq!(closing_field_colon(":é:a"), 3);
}
