use task_2469::*;

#[test]
fn public_examples() {
    assert_eq!(reverse_raw_reads_longest_run(&[]), 0);
    assert_eq!(reverse_raw_reads_longest_run(&[4]), 1);
}
