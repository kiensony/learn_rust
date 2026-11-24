use task_479::*;

#[test]
fn public_examples() {
    assert_eq!(closing_field_pipe(("first\n123", "x")), 3);
    assert_eq!(closing_field_pipe(("|é|a", "z|w")), 1);
}
