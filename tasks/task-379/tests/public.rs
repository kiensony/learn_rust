use task_379::*;

#[test]
fn public_examples() {
    assert_eq!(closing_field_pipe(&String::from("first\n123")), 3);
    assert_eq!(closing_field_pipe(&String::from("|é|a")), 1);
}
