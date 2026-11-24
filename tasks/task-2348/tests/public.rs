use task_2348::*;

#[test]
fn public_examples() {
    assert_eq!(
        batch_medians_rolling_load(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 1),
        vec![6u64, 15u64, 8u64, 31u64]
    );
    assert_eq!(batch_medians_rolling_load(&[], 0), vec![]);
}
