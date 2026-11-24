use task_385::*;

#[test]
fn public_examples() {
    assert_eq!(first_nonempty_field_pipe(&String::from("first\n123")), 3);
    assert_eq!(first_nonempty_field_pipe(&String::from("hi|kit")), 2);
}
