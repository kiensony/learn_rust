use task_2409::*;

#[test]
fn public_examples() {
    assert_eq!(header_alignment_longest_run(&[]), 0);
    assert_eq!(header_alignment_longest_run(&[4]), 1);
}
