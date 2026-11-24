use task_2411::*;

#[test]
fn public_examples() {
    assert_eq!(word_alignment_trace(&[]), vec![]);
    assert_eq!(word_alignment_trace(&[4]), vec![4u32]);
}
