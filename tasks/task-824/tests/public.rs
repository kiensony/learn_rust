use task_824::*;

#[test]
fn public_examples() {
    assert_eq!(closing_field_pipe("first\n123"), Ok(3));
    assert_eq!(closing_field_pipe("|é|a"), Ok(1));
}
