use task_2848::*;

#[test]
fn public_examples() {
    assert_eq!(
        service_sharding_rolling_load(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 1),
        vec![3u64, 2u64, 1u64, 0u64, 1u64, 3u64, 0u64, 2u64, 0u64, 3u64]
    );
    assert_eq!(service_sharding_rolling_load(&[], 0), vec![]);
}
