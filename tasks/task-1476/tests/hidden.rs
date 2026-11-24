use task_1476::*;

#[test]
fn edge_cases() {
    assert_eq!(retry_fixture_descending_steps("200 503 404 200"), 0);
    assert_eq!(retry_fixture_descending_steps("xx 20 099 201 600"), 0);
    assert_eq!(retry_fixture_descending_steps("500 500 204 301 404"), 0);
    assert_eq!(retry_fixture_descending_steps(" 302 204 503 400 201 "), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1476::TRACK_PROOF, 0x52540000 | 1476);
}
