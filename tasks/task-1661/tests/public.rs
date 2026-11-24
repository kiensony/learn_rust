use task_1661::*;

#[test]
fn public_examples() {
    assert_eq!(minor_version_trace(""), vec![]);
    assert_eq!(minor_version_trace("0.0.1"), vec![0u32]);
}
