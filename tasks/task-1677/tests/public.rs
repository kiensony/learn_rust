use task_1677::*;

#[test]
fn public_examples() {
    assert_eq!(patch_version_distinct(""), vec![]);
    assert_eq!(patch_version_distinct("0.0.1"), vec![1u32]);
}
