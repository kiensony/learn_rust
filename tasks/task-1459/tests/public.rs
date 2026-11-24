use task_1459::*;

#[test]
fn public_examples() {
    assert_eq!(http_status_fixture_longest_run(""), 0);
    assert_eq!(http_status_fixture_longest_run("200"), 1);
}
