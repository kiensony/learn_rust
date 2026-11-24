use task_1466::*;

#[test]
fn public_examples() {
    assert_eq!(client_error_fixture_descending_steps(""), 0);
    assert_eq!(client_error_fixture_descending_steps("200"), 0);
}
