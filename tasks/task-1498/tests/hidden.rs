use task_1498::*;

#[test]
fn edge_cases() {
    assert_eq!(retry_fixture_rolling_load("200", 1), vec![]);
    assert_eq!(retry_fixture_rolling_load("200 503 404 200", 2), vec![]);
    assert_eq!(retry_fixture_rolling_load("xx 20 099 201 600", 0), vec![]);
    assert_eq!(
        retry_fixture_rolling_load("500 500 204 301 404", 100),
        vec![]
    );
    assert_eq!(
        retry_fixture_rolling_load(" 302 204 503 400 201 ", 3),
        vec![]
    );
    assert_eq!(
        retry_fixture_rolling_load("500 500 204 301 404", 2),
        vec![1000u64]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1498::TRACK_PROOF, 0x52540000 | 1498);
}
