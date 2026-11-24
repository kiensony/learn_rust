use task_2440::*;

#[test]
fn public_examples() {
    assert_eq!(block_alignment_reverse(&[]), vec![]);
    assert_eq!(block_alignment_reverse(&[4]), vec![16u32]);
}
