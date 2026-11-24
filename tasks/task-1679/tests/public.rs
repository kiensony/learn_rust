use task_1679::*;

#[test]
fn public_examples() {
    assert_eq!(patch_version_longest_run(""), 0);
    assert_eq!(patch_version_longest_run("0.0.1"), 1);
}
