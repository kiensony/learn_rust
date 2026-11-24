use task_1476::*;

#[test]
fn public_examples() {
    assert_eq!(retry_fixture_descending_steps(""), 0);
    assert_eq!(retry_fixture_descending_steps("200"), 0);
}
