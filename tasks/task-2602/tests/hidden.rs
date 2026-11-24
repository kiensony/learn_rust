use task_2602::*;

#[test]
fn edge_cases() {
    assert_eq!(embedded_checksum_total(&[9, 2, 2, 8, 0, 5]), 268);
    assert_eq!(embedded_checksum_total(&[0, 0, 0]), 0);
    assert_eq!(embedded_checksum_total(&[255, 1, 255, 2]), 639);
    assert_eq!(
        embedded_checksum_total(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        703
    );
    assert_eq!(embedded_checksum_total(&[17, 17, 1, 0, 2]), 401);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2602::TRACK_PROOF, 0x52540000 | 2602);
}
