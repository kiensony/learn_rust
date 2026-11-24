use task_1655::*;

#[test]
fn public_examples() {
    assert_eq!(major_version_changes(""), vec![]);
    assert_eq!(major_version_changes("0.0.1"), vec![0u32]);
}
