use task_881::*;

#[test]
fn public_examples() {
    assert_eq!(longest_field_with_last_tie_pipe("first\n123"), 5);
    assert_eq!(longest_field_with_last_tie_pipe("|é|a"), 4);
}
