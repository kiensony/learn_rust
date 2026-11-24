use task_2449::*;

#[test]
fn public_examples() {
    assert_eq!(
        block_alignment_frequent_readings(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 3),
        vec![16u32, 32u32, 0u32]
    );
    assert_eq!(block_alignment_frequent_readings(&[], 0), vec![]);
}
