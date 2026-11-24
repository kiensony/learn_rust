use task_1498::*;

#[test]
fn public_examples() {
    assert_eq!(
        retry_fixture_rolling_load("500 500 204 301 404", 1),
        vec![500u64, 500u64]
    );
    assert_eq!(retry_fixture_rolling_load("", 0), vec![]);
}
