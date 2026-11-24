use task_1471::*;

#[test]
fn public_examples() {
    assert_eq!(retry_fixture_trace(""), vec![]);
    assert_eq!(retry_fixture_trace("200"), vec![]);
}
