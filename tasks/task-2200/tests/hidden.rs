use task_2200::*;

#[test]
fn edge_cases() {
    assert_eq!(duplicate_insertion_cost_replay_mismatch(&[4], &[1]), None);
    assert_eq!(
        duplicate_insertion_cost_replay_mismatch(&[9, 2, 2, 8, 0, 5], &[1, 2, 2, 3, 4, 5, 777]),
        Some(6)
    );
    assert_eq!(
        duplicate_insertion_cost_replay_mismatch(&[0, 0, 0], &[1, 1]),
        Some(2)
    );
    assert_eq!(
        duplicate_insertion_cost_replay_mismatch(&[255, 1, 255, 2], &[9999]),
        Some(0)
    );
    assert_eq!(
        duplicate_insertion_cost_replay_mismatch(
            &[3, 6, 9, 12, 21, 15, 0, 26, 8, 31],
            &[10, 9, 8, 7, 6, 5, 4, 3, 2, 1]
        ),
        Some(0)
    );
    assert_eq!(
        duplicate_insertion_cost_replay_mismatch(&[17, 17, 1, 0, 2], &[1, 1, 2, 3, 4]),
        None
    );
    assert_eq!(
        duplicate_insertion_cost_replay_mismatch(
            &[3, 6, 9, 12, 21, 15, 0, 26, 8, 31],
            &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1]
        ),
        Some(10)
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2200::TRACK_PROOF, 0x52540000 | 2200);
}
