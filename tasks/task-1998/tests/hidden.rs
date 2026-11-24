use task_1998::*;

#[test]
fn edge_cases() {
    assert_eq!(atomic_toggle_rolling_load(&[4], 1), vec![0u64]);
    assert_eq!(
        atomic_toggle_rolling_load(&[9, 2, 2, 8, 0, 5], 2),
        vec![9u64, 20u64, 20u64, 10u64, 2u64]
    );
    assert_eq!(atomic_toggle_rolling_load(&[0, 0, 0], 0), vec![]);
    assert_eq!(atomic_toggle_rolling_load(&[255, 1, 255, 2], 100), vec![]);
    assert_eq!(
        atomic_toggle_rolling_load(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 3),
        vec![8u64, 20u64, 17u64, 33u64, 47u64, 73u64, 52u64, 34u64]
    );
    assert_eq!(
        atomic_toggle_rolling_load(&[17, 17, 1, 0, 2], 1),
        vec![0u64, 17u64, 0u64, 1u64, 1u64]
    );
    assert_eq!(
        atomic_toggle_rolling_load(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 2),
        vec![3u64, 8u64, 17u64, 12u64, 21u64, 47u64, 52u64, 26u64, 8u64]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1998::TRACK_PROOF, 0x52540000 | 1998);
}
