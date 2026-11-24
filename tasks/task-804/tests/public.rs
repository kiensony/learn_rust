use task_804::*;

#[test]
fn public_examples() {
    assert_eq!(closing_field_colon("hi:kit"), Ok(3));
    assert_eq!(closing_field_colon(":é:a"), Ok(1));
}
