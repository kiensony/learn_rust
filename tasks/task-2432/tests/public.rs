use task_2432::*;

#[test]
fn public_examples() {
    assert_eq!(block_alignment_total(&[]), 0);
    assert_eq!(block_alignment_total(&[4]), 16);
}
