use task_1452::*;

#[test]
fn edge_cases() {
    assert_eq!(http_status_fixture_total("200 503 404 200"), 13);
    assert_eq!(http_status_fixture_total("xx 20 099 201 600"), 2);
    assert_eq!(http_status_fixture_total("500 500 204 301 404"), 19);
    assert_eq!(http_status_fixture_total(" 302 204 503 400 201 "), 16);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1452::TRACK_PROOF, 0x52540000 | 1452);
}
