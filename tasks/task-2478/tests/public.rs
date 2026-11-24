use task_2478::*;

#[test]
fn public_examples() {
    assert_eq!(strided_raw_reads_steps(&[]), vec![]);
    assert_eq!(strided_raw_reads_steps(&[4]), vec![]);
}
