use task_2456::*;

#[test]
fn public_examples() {
    assert_eq!(forward_raw_reads_descending_steps(&[]), 0);
    assert_eq!(forward_raw_reads_descending_steps(&[4]), 0);
}
