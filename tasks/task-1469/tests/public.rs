use task_1469::*;

#[test]
fn public_examples() {
    assert_eq!(client_error_fixture_longest_run(""), 0);
    assert_eq!(client_error_fixture_longest_run("200"), 0);
}
