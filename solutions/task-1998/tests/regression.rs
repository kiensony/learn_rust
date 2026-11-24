use task_1998::*;

#[test]
fn transfer_case() {
    assert_eq!(
        atomic_toggle_rolling_load(
            &[
                3, 6, 9, 12, 21, 15, 0, 26, 8, 31, 31, 8, 26, 0, 15, 21, 12, 9, 6, 3
            ],
            3
        ),
        vec![
            8u64, 20u64, 17u64, 33u64, 47u64, 73u64, 52u64, 34u64, 31u64, 39u64, 31u64, 34u64,
            52u64, 73u64, 47u64, 33u64, 17u64, 20u64
        ]
    );
}
