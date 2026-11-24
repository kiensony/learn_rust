use task_2050::*;

#[test]
fn edge_cases() {
    assert_eq!(polled_costs_replay_mismatch(&[4], &[1]), None);
    assert_eq!(
        polled_costs_replay_mismatch(&[9, 2, 2, 8, 0, 5], &[2, 1, 1, 1, 0, 2, 777]),
        Some(6)
    );
    assert_eq!(polled_costs_replay_mismatch(&[0, 0, 0], &[0, 0]), Some(2));
    assert_eq!(
        polled_costs_replay_mismatch(&[255, 1, 255, 2], &[9999]),
        Some(0)
    );
    assert_eq!(
        polled_costs_replay_mismatch(
            &[3, 6, 9, 12, 21, 15, 0, 26, 8, 31],
            &[5, 1, 3, 0, 4, 3, 2, 2, 2, 2]
        ),
        Some(0)
    );
    assert_eq!(
        polled_costs_replay_mismatch(&[17, 17, 1, 0, 2], &[2, 2, 1, 0, 1]),
        None
    );
    assert_eq!(
        polled_costs_replay_mismatch(
            &[3, 6, 9, 12, 21, 15, 0, 26, 8, 31],
            &[2, 2, 2, 2, 3, 4, 0, 3, 1, 5, 1]
        ),
        Some(10)
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2050::TRACK_PROOF, 0x52540000 | 2050);
}
