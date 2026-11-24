use task_1250::*;

#[test]
fn edge_cases() {
    assert_eq!(limited_reads_replay_mismatch(&[4], &[4]), None);
    assert_eq!(
        limited_reads_replay_mismatch(&[9, 2, 2, 8, 0, 5], &[9, 2, 2, 8, 777]),
        Some(4)
    );
    assert_eq!(limited_reads_replay_mismatch(&[0, 0, 0], &[0, 0]), Some(2));
    assert_eq!(
        limited_reads_replay_mismatch(&[255, 1, 255, 2], &[9999]),
        Some(0)
    );
    assert_eq!(
        limited_reads_replay_mismatch(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], &[12, 9, 6, 3]),
        Some(0)
    );
    assert_eq!(
        limited_reads_replay_mismatch(&[17, 17, 1, 0, 2], &[17, 17, 1, 0]),
        None
    );
    assert_eq!(
        limited_reads_replay_mismatch(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], &[3, 6, 9, 12, 1]),
        Some(4)
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1250::TRACK_PROOF, 0x52540000 | 1250);
}
