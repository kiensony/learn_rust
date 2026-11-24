use task_2448::*;

#[test]
fn public_examples() {
    assert_eq!(
        wide_alignment_rolling_load(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 1),
        vec![
            8u64, 8u64, 16u64, 16u64, 24u64, 16u64, 0u64, 32u64, 8u64, 32u64
        ]
    );
    assert_eq!(wide_alignment_rolling_load(&[], 0), vec![]);
}
