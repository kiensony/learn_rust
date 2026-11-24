use task_2452::*;

#[test]
fn public_examples() {
    assert_eq!(forward_raw_reads_total(&[]), 0);
    assert_eq!(forward_raw_reads_total(&[4]), 4);
}
