use task_1446::*;

#[test]
fn edge_cases() {
    assert_eq!(
        millisecond_remainder_threshold_crossing(&[4], 0),
        Some((0, 68))
    );
    assert_eq!(
        millisecond_remainder_threshold_crossing(&[9, 2, 2, 8, 0, 5], 1),
        Some((0, 153))
    );
    assert_eq!(
        millisecond_remainder_threshold_crossing(&[0, 0, 0], 0),
        Some((0, 0))
    );
    assert_eq!(
        millisecond_remainder_threshold_crossing(&[255, 1, 255, 2], 256),
        Some((0, 335))
    );
    assert_eq!(
        millisecond_remainder_threshold_crossing(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 0),
        Some((0, 51))
    );
    assert_eq!(
        millisecond_remainder_threshold_crossing(&[17, 17, 1, 0, 2], 1),
        Some((0, 289))
    );
    assert_eq!(
        millisecond_remainder_threshold_crossing(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 528),
        None
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1446::TRACK_PROOF, 0x52540000 | 1446);
}
