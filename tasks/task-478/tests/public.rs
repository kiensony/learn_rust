use task_478::*;

#[test]
fn public_examples() {
    assert_eq!(leading_field_pipe(("first\n123", "x")), 3);
    assert_eq!(leading_field_pipe(("hi|kit", "x")), 2);
}
