use task_2848::*;

#[test]
fn transfer_case() {
    assert_eq!(
        service_sharding_rolling_load(
            &[
                3, 6, 9, 12, 21, 15, 0, 26, 8, 31, 31, 8, 26, 0, 15, 21, 12, 9, 6, 3
            ],
            3
        ),
        vec![
            6u64, 3u64, 2u64, 4u64, 4u64, 5u64, 2u64, 5u64, 6u64, 6u64, 5u64, 2u64, 5u64, 4u64,
            4u64, 2u64, 3u64, 6u64
        ]
    );
}
