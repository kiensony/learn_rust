use task_1792::*;

#[test]
fn edge_cases() {
    assert_eq!(token_offset_total(&[9, 2, 2, 8, 0, 5]), 56);
    assert_eq!(token_offset_total(&[0, 0, 0]), 15);
    assert_eq!(token_offset_total(&[255, 1, 255, 2]), 533);
    assert_eq!(
        token_offset_total(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        181
    );
    assert_eq!(token_offset_total(&[17, 17, 1, 0, 2]), 62);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1792::TRACK_PROOF, 0x52540000 | 1792);
}
