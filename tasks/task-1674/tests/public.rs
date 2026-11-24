use task_1674::*;

#[test]
fn public_examples() {
    assert_eq!(patch_version_peaks(""), vec![]);
    assert_eq!(patch_version_peaks("0.0.1"), vec![1u32]);
}
