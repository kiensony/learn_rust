use task_1472::*;

#[test]
fn edge_cases() {
    assert_eq!(retry_fixture_total("200 503 404 200"), 503);
    assert_eq!(retry_fixture_total("xx 20 099 201 600"), 0);
    assert_eq!(retry_fixture_total("500 500 204 301 404"), 1000);
    assert_eq!(retry_fixture_total(" 302 204 503 400 201 "), 503);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1472::TRACK_PROOF, 0x52540000 | 1472);
}
