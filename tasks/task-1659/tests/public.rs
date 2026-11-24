use task_1659::*;

#[test]
fn public_examples() {
    assert_eq!(major_version_longest_run(""), 0);
    assert_eq!(major_version_longest_run("0.0.1"), 1);
}
