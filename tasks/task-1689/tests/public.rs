use task_1689::*;

#[test]
fn public_examples() {
    assert_eq!(preview_version_longest_run(""), 0);
    assert_eq!(preview_version_longest_run("0.0.1"), 1);
}
