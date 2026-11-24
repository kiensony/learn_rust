use task_2498::*;

#[test]
fn transfer_case() {
    assert_eq!(
        strided_raw_reads_rolling_load(
            &[
                3, 6, 9, 12, 21, 15, 0, 26, 8, 31, 31, 8, 26, 0, 15, 21, 12, 9, 6, 3
            ],
            3
        ),
        vec![33u64, 30u64, 29u64, 39u64, 65u64, 72u64, 53u64, 33u64]
    );
}
