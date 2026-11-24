use task_1687::*;

#[test]
fn public_examples() {
    assert_eq!(preview_version_distinct(""), vec![]);
    assert_eq!(preview_version_distinct("0.0.1"), vec![1u32]);
}
