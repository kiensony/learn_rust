use task_1669::*;

#[test]
fn public_examples() {
    assert_eq!(minor_version_longest_run(""), 0);
    assert_eq!(minor_version_longest_run("0.0.1"), 1);
}
