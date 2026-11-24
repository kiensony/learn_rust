use task_2348::*;

#[test]
fn transfer_case() {
    assert_eq!(
        batch_medians_rolling_load(
            &[
                3, 6, 9, 12, 21, 15, 0, 26, 8, 31, 31, 8, 26, 0, 15, 21, 12, 9, 6, 3
            ],
            3
        ),
        vec![29u64, 54u64, 54u64, 58u64, 30u64]
    );
}
