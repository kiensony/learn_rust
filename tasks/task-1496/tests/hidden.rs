use task_1496::*;

#[test]
fn edge_cases() {
    assert_eq!(
        http_status_fixture_threshold_crossing("200", 0),
        Some((0, 2))
    );
    assert_eq!(
        http_status_fixture_threshold_crossing("200 503 404 200", 1),
        Some((0, 2))
    );
    assert_eq!(
        http_status_fixture_threshold_crossing("xx 20 099 201 600", 2),
        Some((0, 2))
    );
    assert_eq!(
        http_status_fixture_threshold_crossing("500 500 204 301 404", 256),
        None
    );
    assert_eq!(
        http_status_fixture_threshold_crossing(" 302 204 503 400 201 ", 0),
        Some((0, 3))
    );
    assert_eq!(
        http_status_fixture_threshold_crossing("500 500 204 301 404", 6),
        None
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1496::TRACK_PROOF, 0x52540000 | 1496);
}
