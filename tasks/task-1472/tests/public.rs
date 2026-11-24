use task_1472::*;

#[test]
fn public_examples() {
    assert_eq!(retry_fixture_total(""), 0);
    assert_eq!(retry_fixture_total("200"), 0);
}
