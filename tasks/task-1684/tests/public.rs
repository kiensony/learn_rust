use task_1684::*;

#[test]
fn public_examples() {
    assert_eq!(preview_version_peaks(""), vec![]);
    assert_eq!(preview_version_peaks("0.0.1"), vec![1u32]);
}
