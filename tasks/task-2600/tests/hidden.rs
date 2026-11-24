use task_2600::*;

#[test]
fn edge_cases() {
    assert_eq!(interior_stride_proof_replay_mismatch(&[4], &[]), None);
    assert_eq!(
        interior_stride_proof_replay_mismatch(&[9, 2, 2, 8, 0, 5], &[3, 6, 777]),
        Some(2)
    );
    assert_eq!(
        interior_stride_proof_replay_mismatch(&[0, 0, 0], &[]),
        Some(0)
    );
    assert_eq!(
        interior_stride_proof_replay_mismatch(&[255, 1, 255, 2], &[9999]),
        Some(0)
    );
    assert_eq!(
        interior_stride_proof_replay_mismatch(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], &[9, 16, 10]),
        Some(0)
    );
    assert_eq!(
        interior_stride_proof_replay_mismatch(&[17, 17, 1, 0, 2], &[2]),
        None
    );
    assert_eq!(
        interior_stride_proof_replay_mismatch(
            &[3, 6, 9, 12, 21, 15, 0, 26, 8, 31],
            &[10, 16, 9, 1]
        ),
        Some(3)
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2600::TRACK_PROOF, 0x52540000 | 2600);
}
