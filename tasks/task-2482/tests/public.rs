use task_2482::*;

#[test]
fn public_examples() {
    assert_eq!(trailing_raw_reads_total(&[]), 0);
    assert_eq!(trailing_raw_reads_total(&[4]), 4);
}
