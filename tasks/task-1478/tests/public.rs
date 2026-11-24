use task_1478::*;

#[test]
fn public_examples() {
    assert_eq!(retry_fixture_steps(""), vec![]);
    assert_eq!(retry_fixture_steps("200"), vec![]);
}
