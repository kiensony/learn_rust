use task_1665::*;

#[test]
fn public_examples() {
    assert_eq!(minor_version_changes(""), vec![]);
    assert_eq!(minor_version_changes("0.0.1"), vec![0u32]);
}
