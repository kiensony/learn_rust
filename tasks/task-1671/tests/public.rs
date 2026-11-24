use task_1671::*;

#[test]
fn public_examples() {
    assert_eq!(patch_version_trace(""), vec![]);
    assert_eq!(patch_version_trace("0.0.1"), vec![1u32]);
}
