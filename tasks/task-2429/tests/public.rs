use task_2429::*;

#[test]
fn public_examples() {
    assert_eq!(wide_alignment_longest_run(&[]), 0);
    assert_eq!(wide_alignment_longest_run(&[4]), 1);
}
