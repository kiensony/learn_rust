use task_1550::*;

#[test]
fn edge_cases() {
    assert_eq!(packet_reversal_replay_mismatch(&[4], &[32]), None);
    assert_eq!(
        packet_reversal_replay_mismatch(&[9, 2, 2, 8, 0, 5], &[144, 64, 64, 16, 0, 160, 777]),
        Some(6)
    );
    assert_eq!(
        packet_reversal_replay_mismatch(&[0, 0, 0], &[0, 0]),
        Some(2)
    );
    assert_eq!(
        packet_reversal_replay_mismatch(&[255, 1, 255, 2], &[9999]),
        Some(0)
    );
    assert_eq!(
        packet_reversal_replay_mismatch(
            &[3, 6, 9, 12, 21, 15, 0, 26, 8, 31],
            &[248, 16, 88, 0, 240, 168, 48, 144, 96, 192]
        ),
        Some(0)
    );
    assert_eq!(
        packet_reversal_replay_mismatch(&[17, 17, 1, 0, 2], &[136, 136, 128, 0, 64]),
        None
    );
    assert_eq!(
        packet_reversal_replay_mismatch(
            &[3, 6, 9, 12, 21, 15, 0, 26, 8, 31],
            &[192, 96, 144, 48, 168, 240, 0, 88, 16, 248, 1]
        ),
        Some(10)
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1550::TRACK_PROOF, 0x52540000 | 1550);
}
