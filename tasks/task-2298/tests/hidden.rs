use task_2298::*;

#[test]
fn edge_cases() {
    assert_eq!(exclusive_hotspots_rolling_load(&[4], 1), vec![4u64]);
    assert_eq!(
        exclusive_hotspots_rolling_load(&[9, 2, 2, 8, 0, 5], 2),
        vec![5u64, 13u64, 17u64]
    );
    assert_eq!(exclusive_hotspots_rolling_load(&[0, 0, 0], 0), vec![]);
    assert_eq!(
        exclusive_hotspots_rolling_load(&[255, 1, 255, 2], 100),
        vec![]
    );
    assert_eq!(
        exclusive_hotspots_rolling_load(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 3),
        vec![9u64, 17u64, 23u64, 29u64, 36u64, 48u64, 62u64, 78u64]
    );
    assert_eq!(
        exclusive_hotspots_rolling_load(&[17, 17, 1, 0, 2], 1),
        vec![0u64, 1u64, 2u64]
    );
    assert_eq!(
        exclusive_hotspots_rolling_load(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 2),
        vec![3u64, 9u64, 14u64, 17u64, 21u64, 27u64, 36u64, 47u64, 57u64]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2298::TRACK_PROOF, 0x52540000 | 2298);
}
