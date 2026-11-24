use task_2431::*;

#[test]
fn public_examples() {
    assert_eq!(block_alignment_trace(&[]), vec![]);
    assert_eq!(block_alignment_trace(&[4]), vec![16u32]);
}
