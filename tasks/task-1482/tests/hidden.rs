use task_1482::*;

#[test]
fn edge_cases() {
    assert_eq!(response_boundaries_total("200 503 404 200"), 3);
    assert_eq!(response_boundaries_total("xx 20 099 201 600"), 0);
    assert_eq!(response_boundaries_total("500 500 204 301 404"), 5);
    assert_eq!(response_boundaries_total(" 302 204 503 400 201 "), 5);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1482::TRACK_PROOF, 0x52540000 | 1482);
}
