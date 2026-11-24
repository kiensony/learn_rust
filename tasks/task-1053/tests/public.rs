use task_1053::*;

#[test]
fn public_examples() {
    assert_eq!(closing_field_colon(&"hi:kit"), 3);
    assert_eq!(closing_field_colon(&String::from(":é:a")), 1);
}
