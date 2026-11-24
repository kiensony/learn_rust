use task_315::*;

#[test]
fn public_examples() {
    assert_eq!(first_nonempty_field_colon(String::from("hi:kit")), "hi");
    assert_eq!(first_nonempty_field_colon(String::from(":é:a")), "é");
}
