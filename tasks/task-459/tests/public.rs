use task_459::*;

#[test]
fn public_examples() {
    assert_eq!(closing_field_colon(("hi:kit", "x")), 3);
    assert_eq!(closing_field_colon((":é:a", "z:w")), 1);
}
