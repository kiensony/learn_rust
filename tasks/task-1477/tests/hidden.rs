use task_1477::*;

#[test]
fn edge_cases() {
    assert_eq!(retry_fixture_distinct("200 503 404 200"), vec![503u32]);
    assert_eq!(retry_fixture_distinct("xx 20 099 201 600"), vec![]);
    assert_eq!(retry_fixture_distinct("500 500 204 301 404"), vec![500u32]);
    assert_eq!(
        retry_fixture_distinct(" 302 204 503 400 201 "),
        vec![503u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1477::TRACK_PROOF, 0x52540000 | 1477);
}
