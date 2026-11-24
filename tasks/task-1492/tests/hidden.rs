use task_1492::*;

#[test]
fn edge_cases() {
    assert_eq!(status_detail_total("200 503 404 200"), 7);
    assert_eq!(status_detail_total("xx 20 099 201 600"), 1);
    assert_eq!(status_detail_total("500 500 204 301 404"), 9);
    assert_eq!(status_detail_total(" 302 204 503 400 201 "), 10);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1492::TRACK_PROOF, 0x52540000 | 1492);
}
