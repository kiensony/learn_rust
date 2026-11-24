use task_2420::*;

#[test]
fn public_examples() {
    assert_eq!(word_alignment_reverse(&[]), vec![]);
    assert_eq!(word_alignment_reverse(&[4]), vec![4u32]);
}
