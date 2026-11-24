use task_2459::*;

#[test]
fn public_examples() {
    assert_eq!(forward_raw_reads_longest_run(&[]), 0);
    assert_eq!(forward_raw_reads_longest_run(&[4]), 1);
}
