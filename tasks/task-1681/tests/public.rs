use task_1681::*;

#[test]
fn public_examples() {
    assert_eq!(preview_version_trace(""), vec![]);
    assert_eq!(preview_version_trace("0.0.1"), vec![1u32]);
}
