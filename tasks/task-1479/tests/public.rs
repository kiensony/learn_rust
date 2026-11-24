use task_1479::*;

#[test]
fn public_examples() {
    assert_eq!(retry_fixture_longest_run(""), 0);
    assert_eq!(retry_fixture_longest_run("200"), 0);
}
