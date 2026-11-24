use task_1396::*;

#[test]
fn edge_cases() {
    assert_eq!(frame_lengths_threshold_crossing(&[0], 0), Some((0, 0)));
    assert_eq!(
        frame_lengths_threshold_crossing(&[2, 7, 3, 1, 4, 0, 3, 9, 9, 2], 1),
        Some((0, 2))
    );
    assert_eq!(frame_lengths_threshold_crossing(&[3, 1], 0), None);
    assert_eq!(
        frame_lengths_threshold_crossing(&[1, 0, 0, 1, 255], 256),
        None
    );
    assert_eq!(
        frame_lengths_threshold_crossing(&[2, 3, 1, 2, 3, 1, 1, 2], 0),
        Some((0, 2))
    );
    assert_eq!(frame_lengths_threshold_crossing(&[255, 0], 1), None);
    assert_eq!(
        frame_lengths_threshold_crossing(&[2, 7, 3, 1, 4, 0, 3, 9, 9, 2], 4),
        None
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1396::TRACK_PROOF, 0x52540000 | 1396);
}
