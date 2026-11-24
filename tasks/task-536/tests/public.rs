use task_536::*;

#[test]
fn public_examples() {
    assert_eq!(longest_field_with_last_tie_pipe("first\n123"), 3);
    assert_eq!(longest_field_with_last_tie_pipe("|é|a"), 1);
}
