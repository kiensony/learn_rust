use task_2412::*;

#[test]
fn public_examples() {
    assert_eq!(word_alignment_total(&[]), 0);
    assert_eq!(word_alignment_total(&[4]), 4);
}
