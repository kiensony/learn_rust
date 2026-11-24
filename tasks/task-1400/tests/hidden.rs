use task_1400::*;

#[test]
fn edge_cases() {
    assert_eq!(frame_maxima_replay_mismatch(&[0], &[0]), None);
    assert_eq!(
        frame_maxima_replay_mismatch(&[2, 7, 3, 1, 4, 0, 3, 9, 9, 2], &[7, 4, 0, 9, 777]),
        Some(4)
    );
    assert_eq!(frame_maxima_replay_mismatch(&[3, 1], &[]), None);
    assert_eq!(
        frame_maxima_replay_mismatch(&[1, 0, 0, 1, 255], &[9999]),
        Some(0)
    );
    assert_eq!(
        frame_maxima_replay_mismatch(&[2, 3, 1, 2, 3, 1, 1, 2], &[2, 3, 3]),
        Some(0)
    );
    assert_eq!(frame_maxima_replay_mismatch(&[255, 0], &[]), None);
    assert_eq!(
        frame_maxima_replay_mismatch(&[2, 7, 3, 1, 4, 0, 3, 9, 9, 2], &[7, 4, 0, 9, 1]),
        Some(4)
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1400::TRACK_PROOF, 0x52540000 | 1400);
}
