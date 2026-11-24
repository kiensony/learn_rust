use task_1488::*;

#[test]
fn edge_cases() {
    assert_eq!(response_boundaries_steps("200 503 404 200"), vec![3u32]);
    assert_eq!(response_boundaries_steps("xx 20 099 201 600"), vec![]);
    assert_eq!(response_boundaries_steps("500 500 204 301 404"), vec![1u32]);
    assert_eq!(
        response_boundaries_steps(" 302 204 503 400 201 "),
        vec![1u32, 3u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1488::TRACK_PROOF, 0x52540000 | 1488);
}
