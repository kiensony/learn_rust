use task_2300::*;

#[test]
fn edge_cases() {
    assert_eq!(profile_ranking_replay_mismatch(&[4], &[4]), None);
    assert_eq!(
        profile_ranking_replay_mismatch(&[9, 2, 2, 8, 0, 5], &[2, 9, 8, 5, 0, 777]),
        Some(5)
    );
    assert_eq!(profile_ranking_replay_mismatch(&[0, 0, 0], &[]), Some(0));
    assert_eq!(
        profile_ranking_replay_mismatch(&[255, 1, 255, 2], &[9999]),
        Some(0)
    );
    assert_eq!(
        profile_ranking_replay_mismatch(
            &[3, 6, 9, 12, 21, 15, 0, 26, 8, 31],
            &[0, 3, 6, 8, 9, 12, 15, 21, 26, 31]
        ),
        Some(0)
    );
    assert_eq!(
        profile_ranking_replay_mismatch(&[17, 17, 1, 0, 2], &[17, 2, 1, 0]),
        None
    );
    assert_eq!(
        profile_ranking_replay_mismatch(
            &[3, 6, 9, 12, 21, 15, 0, 26, 8, 31],
            &[31, 26, 21, 15, 12, 9, 8, 6, 3, 0, 1]
        ),
        Some(10)
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2300::TRACK_PROOF, 0x52540000 | 2300);
}
