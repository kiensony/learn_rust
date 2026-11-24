use task_2472::*;

#[test]
fn public_examples() {
    assert_eq!(strided_raw_reads_total(&[]), 0);
    assert_eq!(strided_raw_reads_total(&[4]), 4);
}
