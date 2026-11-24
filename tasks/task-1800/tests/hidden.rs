use task_1800::*;

#[test]
fn edge_cases() {
    assert_eq!(token_offset_replay_mismatch(&[4], &[9]), None);
    assert_eq!(
        token_offset_replay_mismatch(&[9, 2, 2, 8, 0, 5], &[14, 7, 7, 13, 5, 10, 777]),
        Some(6)
    );
    assert_eq!(token_offset_replay_mismatch(&[0, 0, 0], &[5, 5]), Some(2));
    assert_eq!(
        token_offset_replay_mismatch(&[255, 1, 255, 2], &[9999]),
        Some(0)
    );
    assert_eq!(
        token_offset_replay_mismatch(
            &[3, 6, 9, 12, 21, 15, 0, 26, 8, 31],
            &[36, 13, 31, 5, 20, 26, 17, 14, 11, 8]
        ),
        Some(0)
    );
    assert_eq!(
        token_offset_replay_mismatch(&[17, 17, 1, 0, 2], &[22, 22, 6, 5, 7]),
        None
    );
    assert_eq!(
        token_offset_replay_mismatch(
            &[3, 6, 9, 12, 21, 15, 0, 26, 8, 31],
            &[8, 11, 14, 17, 26, 20, 5, 31, 13, 36, 1]
        ),
        Some(10)
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1800::TRACK_PROOF, 0x52540000 | 1800);
}
