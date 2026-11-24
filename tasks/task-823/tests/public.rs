use task_823::*;

#[test]
fn public_examples() {
    assert_eq!(leading_field_pipe("first\n123"), Ok(3));
    assert_eq!(leading_field_pipe("hi|kit"), Ok(2));
}
