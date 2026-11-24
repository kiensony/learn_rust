use task_2417::*;

#[test]
fn public_examples() {
    assert_eq!(word_alignment_distinct(&[]), vec![]);
    assert_eq!(word_alignment_distinct(&[4]), vec![4u32]);
}
