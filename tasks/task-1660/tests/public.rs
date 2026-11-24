use task_1660::*;

#[test]
fn public_examples() {
    assert_eq!(major_version_reverse(""), vec![]);
    assert_eq!(major_version_reverse("0.0.1"), vec![0u32]);
}
