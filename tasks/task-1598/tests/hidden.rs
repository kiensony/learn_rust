use task_1598::*;

#[test]
fn edge_cases() {
    assert_eq!(
        feature_packing_rolling_load(&[4], 1),
        if cfg!(feature = "wide") {
            vec![32u64]
        } else {
            vec![4u64]
        }
    );
    assert_eq!(
        feature_packing_rolling_load(&[9, 2, 2, 8, 0, 5], 2),
        if cfg!(feature = "wide") {
            vec![88u64, 32u64, 80u64, 64u64, 40u64]
        } else {
            vec![11u64, 4u64, 10u64, 8u64, 5u64]
        }
    );
    assert_eq!(feature_packing_rolling_load(&[0, 0, 0], 0), vec![]);
    assert_eq!(feature_packing_rolling_load(&[255, 1, 255, 2], 100), vec![]);
    assert_eq!(
        feature_packing_rolling_load(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 3),
        if cfg!(feature = "wide") {
            vec![
                144u64, 216u64, 336u64, 384u64, 288u64, 328u64, 272u64, 520u64,
            ]
        } else {
            vec![18u64, 27u64, 42u64, 48u64, 36u64, 41u64, 34u64, 65u64]
        }
    );
    assert_eq!(
        feature_packing_rolling_load(&[17, 17, 1, 0, 2], 1),
        if cfg!(feature = "wide") {
            vec![136u64, 136u64, 8u64, 0u64, 16u64]
        } else {
            vec![17u64, 17u64, 1u64, 0u64, 2u64]
        }
    );
    assert_eq!(
        feature_packing_rolling_load(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 2),
        if cfg!(feature = "wide") {
            vec![
                72u64, 120u64, 168u64, 264u64, 288u64, 120u64, 208u64, 272u64, 312u64,
            ]
        } else {
            vec![9u64, 15u64, 21u64, 33u64, 36u64, 15u64, 26u64, 34u64, 39u64]
        }
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1598::TRACK_PROOF, 0x52540000 | 1598);
}
