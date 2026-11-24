use task_1490::*;

#[test]
fn edge_cases() {
    assert_eq!(
        response_boundaries_reverse("200 503 404 200"),
        vec![3u32, 0u32]
    );
    assert_eq!(response_boundaries_reverse("xx 20 099 201 600"), vec![0u32]);
    assert_eq!(
        response_boundaries_reverse("500 500 204 301 404"),
        vec![3u32, 2u32]
    );
    assert_eq!(
        response_boundaries_reverse(" 302 204 503 400 201 "),
        vec![4u32, 1u32, 0u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1490::TRACK_PROOF, 0x52540000 | 1490);
}
