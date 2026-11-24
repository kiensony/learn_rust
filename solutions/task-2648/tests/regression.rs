use task_2648::*;

#[test]
fn transfer_case() {
    assert_eq!(
        embedded_parity_rolling_load(
            &[
                3, 6, 9, 12, 21, 15, 0, 26, 8, 31, 31, 8, 26, 0, 15, 21, 12, 9, 6, 3
            ],
            3
        ),
        vec![
            0u64, 0u64, 1u64, 1u64, 1u64, 1u64, 2u64, 3u64, 3u64, 3u64, 3u64, 2u64, 1u64, 1u64,
            1u64, 1u64, 0u64, 0u64
        ]
    );
}
