use task_2415::*;

#[test]
fn public_examples() {
    assert_eq!(word_alignment_changes(&[]), vec![]);
    assert_eq!(word_alignment_changes(&[4]), vec![4u32]);
}
