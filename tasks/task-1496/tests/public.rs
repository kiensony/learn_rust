use task_1496::*;

#[test]
fn public_examples() {
    assert_eq!(
        http_status_fixture_threshold_crossing("500 500 204 301 404", 0),
        Some((0, 5))
    );
    assert_eq!(http_status_fixture_threshold_crossing("", 0), None);
}
