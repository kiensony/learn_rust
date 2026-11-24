use task_2435::*;

#[test]
fn public_examples() {
    assert_eq!(block_alignment_changes(&[]), vec![]);
    assert_eq!(block_alignment_changes(&[4]), vec![16u32]);
}
