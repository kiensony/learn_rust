use task_1179::*;

#[test]
fn public_examples() {
    assert_eq!(
        longest_field_with_last_tie_pipe("first\n123", "backup"),
        "123"
    );
    assert_eq!(longest_field_with_last_tie_pipe("hi|kit", "backup"), "kit");
}
