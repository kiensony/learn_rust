use task_2048::*;

#[test]
fn transfer_case() {
    assert_eq!(
        polled_groups_rolling_load(
            &[
                3, 6, 9, 12, 21, 15, 0, 26, 8, 31, 31, 8, 26, 0, 15, 21, 12, 9, 6, 3
            ],
            3
        ),
        vec![
            3u64, 6u64, 10u64, 11u64, 8u64, 9u64, 8u64, 15u64, 16u64, 16u64, 15u64, 8u64, 9u64,
            8u64, 11u64, 10u64, 6u64, 3u64
        ]
    );
}
