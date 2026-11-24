use task_2439::*;

#[test]
fn public_examples() {
    assert_eq!(block_alignment_longest_run(&[]), 0);
    assert_eq!(block_alignment_longest_run(&[4]), 1);
}
