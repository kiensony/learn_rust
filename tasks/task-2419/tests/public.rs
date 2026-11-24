use task_2419::*;

#[test]
fn public_examples() {
    assert_eq!(word_alignment_longest_run(&[]), 0);
    assert_eq!(word_alignment_longest_run(&[4]), 1);
}
