use task_2550::*;

#[test]
fn edge_cases() {
    assert_eq!(abi_status_mask_replay_mismatch(&[4], &[4]), None);
    assert_eq!(
        abi_status_mask_replay_mismatch(&[9, 2, 2, 8, 0, 5], &[9, 2, 2, 8, 0, 5, 777]),
        Some(6)
    );
    assert_eq!(
        abi_status_mask_replay_mismatch(&[0, 0, 0], &[0, 0]),
        Some(2)
    );
    assert_eq!(
        abi_status_mask_replay_mismatch(&[255, 1, 255, 2], &[9999]),
        Some(0)
    );
    assert_eq!(
        abi_status_mask_replay_mismatch(
            &[3, 6, 9, 12, 21, 15, 0, 26, 8, 31],
            &[31, 8, 26, 0, 15, 21, 12, 9, 6, 3]
        ),
        Some(0)
    );
    assert_eq!(
        abi_status_mask_replay_mismatch(&[17, 17, 1, 0, 2], &[17, 17, 1, 0, 2]),
        None
    );
    assert_eq!(
        abi_status_mask_replay_mismatch(
            &[3, 6, 9, 12, 21, 15, 0, 26, 8, 31],
            &[3, 6, 9, 12, 21, 15, 0, 26, 8, 31, 1]
        ),
        Some(10)
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2550::TRACK_PROOF, 0x52540000 | 2550);
}
