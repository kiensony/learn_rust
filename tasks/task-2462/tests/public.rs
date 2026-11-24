use task_2462::*;

#[test]
fn public_examples() {
    assert_eq!(reverse_raw_reads_total(&[]), 0);
    assert_eq!(reverse_raw_reads_total(&[4]), 4);
}
