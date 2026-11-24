use task_1690::*;

#[test]
fn public_examples() {
    assert_eq!(preview_version_reverse(""), vec![]);
    assert_eq!(preview_version_reverse("0.0.1"), vec![1u32]);
}
