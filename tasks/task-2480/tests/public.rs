use task_2480::*;

#[test]
fn public_examples() {
    assert_eq!(strided_raw_reads_reverse(&[]), vec![]);
    assert_eq!(strided_raw_reads_reverse(&[4]), vec![4u32]);
}
