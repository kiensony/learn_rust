use task_329::*;

#[test]
fn public_examples() {
    assert_eq!(closing_field_pipe(String::from("first\n123")), "123");
    assert_eq!(closing_field_pipe(String::from("hi|kit")), "kit");
}
