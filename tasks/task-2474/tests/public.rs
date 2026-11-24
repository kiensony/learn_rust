use task_2474::*;

#[test]
fn public_examples() {
    assert_eq!(strided_raw_reads_peaks(&[]), vec![]);
    assert_eq!(strided_raw_reads_peaks(&[4]), vec![4u32]);
}
