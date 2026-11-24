use task_2848::*;

#[test]
fn edge_cases() {
    assert_eq!(service_sharding_rolling_load(&[4], 1), vec![0u64]);
    assert_eq!(
        service_sharding_rolling_load(&[9, 2, 2, 8, 0, 5], 2),
        vec![3u64, 4u64, 2u64, 0u64, 1u64]
    );
    assert_eq!(service_sharding_rolling_load(&[0, 0, 0], 0), vec![]);
    assert_eq!(
        service_sharding_rolling_load(&[255, 1, 255, 2], 100),
        vec![]
    );
    assert_eq!(
        service_sharding_rolling_load(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 3),
        vec![6u64, 3u64, 2u64, 4u64, 4u64, 5u64, 2u64, 5u64]
    );
    assert_eq!(
        service_sharding_rolling_load(&[17, 17, 1, 0, 2], 1),
        vec![1u64, 1u64, 1u64, 0u64, 2u64]
    );
    assert_eq!(
        service_sharding_rolling_load(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 2),
        vec![5u64, 3u64, 1u64, 1u64, 4u64, 3u64, 2u64, 2u64, 3u64]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2848::TRACK_PROOF, 0x52540000 | 2848);
}
