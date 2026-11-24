use task_2450::*;

#[test]
fn edge_cases() {
    assert_eq!(page_alignment_replay_mismatch(&[4], &[32]), None);
    assert_eq!(
        page_alignment_replay_mismatch(&[9, 2, 2, 8, 0, 5], &[32, 32, 32, 32, 0, 32, 777]),
        Some(6)
    );
    assert_eq!(page_alignment_replay_mismatch(&[0, 0, 0], &[0, 0]), Some(2));
    assert_eq!(
        page_alignment_replay_mismatch(&[255, 1, 255, 2], &[9999]),
        Some(0)
    );
    assert_eq!(
        page_alignment_replay_mismatch(
            &[3, 6, 9, 12, 21, 15, 0, 26, 8, 31],
            &[32, 32, 32, 0, 32, 32, 32, 32, 32, 32]
        ),
        Some(3)
    );
    assert_eq!(
        page_alignment_replay_mismatch(&[17, 17, 1, 0, 2], &[32, 32, 32, 0, 32]),
        None
    );
    assert_eq!(
        page_alignment_replay_mismatch(
            &[3, 6, 9, 12, 21, 15, 0, 26, 8, 31],
            &[32, 32, 32, 32, 32, 32, 0, 32, 32, 32, 1]
        ),
        Some(10)
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2450::TRACK_PROOF, 0x52540000 | 2450);
}
