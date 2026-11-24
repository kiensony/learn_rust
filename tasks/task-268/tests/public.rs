use task_268::*;

#[test]
fn public_examples() {
    assert_eq!(normalize_spaces_hash("cat"), "cat");
    assert_eq!(normalize_spaces_hash("#hi#"), "#hi#");
}
