use task_874::*;

#[test]
fn public_examples() {
    assert_eq!(closing_field_pipe("first\n123"), 5);
    assert_eq!(closing_field_pipe("|é|a"), 3);
}
