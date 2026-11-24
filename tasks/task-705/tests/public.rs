use task_705::*;

#[test]
fn public_examples() {
    assert_eq!(leading_field_colon(Some("hi:kit")), None);
    assert_eq!(leading_field_colon(Some("3:72")), Some(3));
}
