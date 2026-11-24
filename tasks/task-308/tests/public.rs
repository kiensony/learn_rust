use task_308::*;

#[test]
fn public_examples() {
    assert_eq!(normalize_spaces_bang("cat"), "cat");
    assert_eq!(normalize_spaces_bang("!hi!"), "!hi!");
}
