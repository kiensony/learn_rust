use task_1462::*;

#[test]
fn edge_cases() {
    assert_eq!(client_error_fixture_total("200 503 404 200"), 404);
    assert_eq!(client_error_fixture_total("xx 20 099 201 600"), 0);
    assert_eq!(client_error_fixture_total("500 500 204 301 404"), 404);
    assert_eq!(client_error_fixture_total(" 302 204 503 400 201 "), 400);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1462::TRACK_PROOF, 0x52540000 | 1462);
}
