use task_1498::*;

#[test]
fn transfer_case() {
    assert_eq!(
        retry_fixture_rolling_load("500 500 204 301 404 500 500 204 301 404", 3),
        vec![1500u64, 1500u64]
    );
}
