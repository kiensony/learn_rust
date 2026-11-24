use task_378::*;

#[test]
fn public_examples() {
    assert_eq!(leading_field_pipe(&String::from("first\n123")), 3);
    assert_eq!(leading_field_pipe(&String::from("hi|kit")), 2);
}
