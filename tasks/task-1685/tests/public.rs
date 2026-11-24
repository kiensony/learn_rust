use task_1685::*;

#[test]
fn public_examples() {
    assert_eq!(preview_version_changes(""), vec![]);
    assert_eq!(preview_version_changes("0.0.1"), vec![1u32]);
}
