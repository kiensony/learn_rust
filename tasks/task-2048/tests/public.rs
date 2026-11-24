use task_2048::*;

#[test]
fn public_examples() {
    assert_eq!(
        polled_groups_rolling_load(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 1),
        vec![0u64, 1u64, 2u64, 3u64, 5u64, 3u64, 0u64, 6u64, 2u64, 7u64]
    );
    assert_eq!(polled_groups_rolling_load(&[], 0), vec![]);
}
