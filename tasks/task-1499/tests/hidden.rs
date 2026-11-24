use task_1499::*;

#[test]
fn edge_cases() {
    assert_eq!(response_boundaries_frequent_readings("200", 0), vec![]);
    assert_eq!(
        response_boundaries_frequent_readings("200 503 404 200", 1),
        vec![0u32]
    );
    assert_eq!(
        response_boundaries_frequent_readings("xx 20 099 201 600", 2),
        vec![0u32]
    );
    assert_eq!(
        response_boundaries_frequent_readings("500 500 204 301 404", 3),
        vec![2u32, 3u32]
    );
    assert_eq!(
        response_boundaries_frequent_readings(" 302 204 503 400 201 ", 256),
        vec![0u32, 1u32, 4u32]
    );
    assert_eq!(
        response_boundaries_frequent_readings(" 302 204 503 400 201 ", 1),
        vec![0u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1499::TRACK_PROOF, 0x52540000 | 1499);
}
