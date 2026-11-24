use task_2464::*;

#[test]
fn public_examples() {
    assert_eq!(reverse_raw_reads_peaks(&[]), vec![]);
    assert_eq!(reverse_raw_reads_peaks(&[4]), vec![4u32]);
}
