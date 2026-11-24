use task_1489::*;

#[test]
fn edge_cases() {
    assert_eq!(response_boundaries_longest_run("200 503 404 200"), 1);
    assert_eq!(response_boundaries_longest_run("xx 20 099 201 600"), 1);
    assert_eq!(response_boundaries_longest_run("500 500 204 301 404"), 1);
    assert_eq!(response_boundaries_longest_run(" 302 204 503 400 201 "), 1);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1489::TRACK_PROOF, 0x52540000 | 1489);
}
