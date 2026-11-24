use task_2500::*;

#[test]
fn edge_cases() {
    assert_eq!(interior_raw_reads_replay_mismatch(&[4], &[]), None);
    assert_eq!(
        interior_raw_reads_replay_mismatch(&[9, 2, 2, 8, 0, 5], &[2, 2, 8, 0, 777]),
        Some(4)
    );
    assert_eq!(interior_raw_reads_replay_mismatch(&[0, 0, 0], &[]), Some(0));
    assert_eq!(
        interior_raw_reads_replay_mismatch(&[255, 1, 255, 2], &[9999]),
        Some(0)
    );
    assert_eq!(
        interior_raw_reads_replay_mismatch(
            &[3, 6, 9, 12, 21, 15, 0, 26, 8, 31],
            &[8, 26, 0, 15, 21, 12, 9, 6]
        ),
        Some(0)
    );
    assert_eq!(
        interior_raw_reads_replay_mismatch(&[17, 17, 1, 0, 2], &[17, 1, 0]),
        None
    );
    assert_eq!(
        interior_raw_reads_replay_mismatch(
            &[3, 6, 9, 12, 21, 15, 0, 26, 8, 31],
            &[6, 9, 12, 21, 15, 0, 26, 8, 1]
        ),
        Some(8)
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2500::TRACK_PROOF, 0x52540000 | 2500);
}
