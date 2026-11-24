use task_310::*;

#[test]
fn public_examples() {
    assert_eq!(closing_field_colon(String::from("hi:kit")), "kit");
    assert_eq!(closing_field_colon(String::from(":é:a")), "a");
}
