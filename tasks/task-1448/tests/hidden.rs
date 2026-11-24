use task_1448::*;

#[test]
fn edge_cases() {
    assert_eq!(remaining_budget_rolling_load(&[4], 1), vec![18u64]);
    assert_eq!(
        remaining_budget_rolling_load(&[9, 2, 2, 8, 0, 5], 2),
        vec![103u64, 0u64, 86u64, 86u64, 35u64]
    );
    assert_eq!(remaining_budget_rolling_load(&[0, 0, 0], 0), vec![]);
    assert_eq!(
        remaining_budget_rolling_load(&[255, 1, 255, 2], 100),
        vec![]
    );
    assert_eq!(
        remaining_budget_rolling_load(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 3),
        vec![
            156u64, 309u64, 564u64, 666u64, 512u64, 597u64, 478u64, 955u64
        ]
    );
    assert_eq!(
        remaining_budget_rolling_load(&[17, 17, 1, 0, 2], 1),
        vec![239u64, 239u64, 0u64, 0u64, 0u64]
    );
    assert_eq!(
        remaining_budget_rolling_load(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 2),
        vec![
            53u64, 155u64, 257u64, 461u64, 512u64, 205u64, 392u64, 478u64, 563u64
        ]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1448::TRACK_PROOF, 0x52540000 | 1448);
}
