use task_458::*;

#[test]
fn public_examples() {
    assert_eq!(leading_field_colon(("hi:kit", "x")), 2);
    assert_eq!(leading_field_colon((":é:a", "z:w")), 0);
}
