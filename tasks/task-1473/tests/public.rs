use task_1473::*;

#[test]
fn public_examples() {
    assert_eq!(retry_fixture_bounds(""), None);
    assert_eq!(retry_fixture_bounds("200"), None);
}
