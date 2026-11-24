use task_2479::*;

#[test]
fn public_examples() {
    assert_eq!(strided_raw_reads_longest_run(&[]), 0);
    assert_eq!(strided_raw_reads_longest_run(&[4]), 1);
}
