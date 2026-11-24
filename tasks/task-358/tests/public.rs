use task_358::*;

#[test]
fn public_examples() {
    assert_eq!(leading_field_colon(&String::from("hi:kit")), 2);
    assert_eq!(leading_field_colon(&String::from(":é:a")), 0);
}
