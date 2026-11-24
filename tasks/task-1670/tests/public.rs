use task_1670::*;

#[test]
fn public_examples() {
    assert_eq!(minor_version_reverse(""), vec![]);
    assert_eq!(minor_version_reverse("0.0.1"), vec![0u32]);
}
