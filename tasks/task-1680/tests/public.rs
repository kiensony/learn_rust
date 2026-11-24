use task_1680::*;

#[test]
fn public_examples() {
    assert_eq!(patch_version_reverse(""), vec![]);
    assert_eq!(patch_version_reverse("0.0.1"), vec![1u32]);
}
