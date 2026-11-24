use task_2498::*;

#[test]
fn public_examples() {
    assert_eq!(
        strided_raw_reads_rolling_load(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 1),
        vec![3u64, 9u64, 21u64, 0u64, 8u64]
    );
    assert_eq!(strided_raw_reads_rolling_load(&[], 0), vec![]);
}
