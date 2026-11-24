use task_1471::*;

#[test]
fn edge_cases() {
    assert_eq!(retry_fixture_trace("200 503 404 200"), vec![503u32]);
    assert_eq!(retry_fixture_trace("xx 20 099 201 600"), vec![]);
    assert_eq!(
        retry_fixture_trace("500 500 204 301 404"),
        vec![500u32, 500u32]
    );
    assert_eq!(retry_fixture_trace(" 302 204 503 400 201 "), vec![503u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1471::TRACK_PROOF, 0x52540000 | 1471);
}
