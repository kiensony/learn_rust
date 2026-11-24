use task_1052::*;

#[test]
fn public_examples() {
    assert_eq!(leading_field_colon(&"hi:kit"), 2);
    assert_eq!(leading_field_colon(&String::from(":é:a")), 0);
}
