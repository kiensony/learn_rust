use task_1496::*;

#[test]
fn transfer_case() {
    assert_eq!(
        http_status_fixture_threshold_crossing("500 500 204 301 404 500 500 204 301 404", 1),
        Some((0, 5))
    );
}
