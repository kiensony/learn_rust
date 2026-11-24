use task_2437::*;

#[test]
fn public_examples() {
    assert_eq!(block_alignment_distinct(&[]), vec![]);
    assert_eq!(block_alignment_distinct(&[4]), vec![16u32]);
}
