use task_1898::*;

#[test]
fn transfer_case() {
    assert_eq!(
        shared_payload_rolling_load(
            &[
                3, 6, 9, 12, 21, 15, 0, 26, 8, 31, 31, 8, 26, 0, 15, 21, 12, 9, 6, 3
            ],
            3
        ),
        vec![
            18u64, 27u64, 42u64, 48u64, 36u64, 41u64, 34u64, 65u64, 70u64, 70u64, 65u64, 34u64,
            41u64, 36u64, 48u64, 42u64, 27u64, 18u64
        ]
    );
}
