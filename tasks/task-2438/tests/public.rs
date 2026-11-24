use task_2438::*;

#[test]
fn public_examples() {
    assert_eq!(block_alignment_steps(&[]), vec![]);
    assert_eq!(block_alignment_steps(&[4]), vec![]);
}
