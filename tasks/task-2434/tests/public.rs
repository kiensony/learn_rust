use task_2434::*;

#[test]
fn public_examples() {
    assert_eq!(block_alignment_peaks(&[]), vec![]);
    assert_eq!(block_alignment_peaks(&[4]), vec![16u32]);
}
