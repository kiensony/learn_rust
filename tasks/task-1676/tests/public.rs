use task_1676::*;

#[test]
fn public_examples() {
    assert_eq!(patch_version_descending_steps(""), 0);
    assert_eq!(patch_version_descending_steps("0.0.1"), 0);
}
