use task_2489::*;

#[test]
fn public_examples() {
    assert_eq!(trailing_raw_reads_longest_run(&[]), 0);
    assert_eq!(trailing_raw_reads_longest_run(&[4]), 1);
}
