use task_529::*;

#[test]
fn public_examples() {
    assert_eq!(closing_field_pipe("first\n123"), 3);
    assert_eq!(closing_field_pipe("|é|a"), 1);
}
