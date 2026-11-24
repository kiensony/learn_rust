use task_1672::*;

#[test]
fn public_examples() {
    assert_eq!(patch_version_total(""), 0);
    assert_eq!(patch_version_total("0.0.1"), 1);
}
