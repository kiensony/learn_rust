use task_1678::*;

#[test]
fn public_examples() {
    assert_eq!(patch_version_steps(""), vec![]);
    assert_eq!(patch_version_steps("0.0.1"), vec![]);
}
