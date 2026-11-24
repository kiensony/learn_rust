use task_1675::*;

#[test]
fn public_examples() {
    assert_eq!(patch_version_changes(""), vec![]);
    assert_eq!(patch_version_changes("0.0.1"), vec![1u32]);
}
