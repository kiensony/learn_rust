use task_1483::*;

#[test]
fn edge_cases() {
    assert_eq!(
        response_boundaries_bounds("200 503 404 200"),
        Some((0u32, 3u32))
    );
    assert_eq!(
        response_boundaries_bounds("xx 20 099 201 600"),
        Some((0u32, 0u32))
    );
    assert_eq!(
        response_boundaries_bounds("500 500 204 301 404"),
        Some((2u32, 3u32))
    );
    assert_eq!(
        response_boundaries_bounds(" 302 204 503 400 201 "),
        Some((0u32, 4u32))
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1483::TRACK_PROOF, 0x52540000 | 1483);
}
