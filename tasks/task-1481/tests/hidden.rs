use task_1481::*;

#[test]
fn edge_cases() {
    assert_eq!(
        response_boundaries_trace("200 503 404 200"),
        vec![0u32, 3u32]
    );
    assert_eq!(response_boundaries_trace("xx 20 099 201 600"), vec![0u32]);
    assert_eq!(
        response_boundaries_trace("500 500 204 301 404"),
        vec![2u32, 3u32]
    );
    assert_eq!(
        response_boundaries_trace(" 302 204 503 400 201 "),
        vec![0u32, 1u32, 4u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1481::TRACK_PROOF, 0x52540000 | 1481);
}
