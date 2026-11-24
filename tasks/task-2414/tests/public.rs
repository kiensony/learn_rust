use task_2414::*;

#[test]
fn public_examples() {
    assert_eq!(word_alignment_peaks(&[]), vec![]);
    assert_eq!(word_alignment_peaks(&[4]), vec![4u32]);
}
