use task_831::*;

#[test]
fn public_examples() {
    assert_eq!(longest_field_with_last_tie_pipe("first\n123"), Ok(3));
    assert_eq!(longest_field_with_last_tie_pipe("|é|a"), Ok(2));
}
