use task_2648::*;

#[test]
fn public_examples() {
    assert_eq!(
        embedded_parity_rolling_load(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 1),
        vec![0u64, 0u64, 0u64, 0u64, 1u64, 0u64, 0u64, 1u64, 1u64, 1u64]
    );
    assert_eq!(embedded_parity_rolling_load(&[], 0), vec![]);
}
