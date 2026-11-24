use task_585::*;

#[test]
fn public_examples() {
    assert_eq!(longest_field_with_last_tie_pipe("first\n123"), "123");
    assert_eq!(longest_field_with_last_tie_pipe("hi|kit"), "kit");
}
