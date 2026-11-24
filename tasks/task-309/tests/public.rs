use task_309::*;

#[test]
fn public_examples() {
    assert_eq!(leading_field_colon(String::from("hi:kit")), "hi");
    assert_eq!(leading_field_colon(String::from(":é:a")), "");
}
