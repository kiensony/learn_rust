use task_1651::*;

#[test]
fn public_examples() {
    assert_eq!(major_version_trace(""), vec![]);
    assert_eq!(major_version_trace("0.0.1"), vec![0u32]);
}
