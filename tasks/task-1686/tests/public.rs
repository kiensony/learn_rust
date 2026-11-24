use task_1686::*;

#[test]
fn public_examples() {
    assert_eq!(preview_version_descending_steps(""), 0);
    assert_eq!(preview_version_descending_steps("0.0.1"), 0);
}
