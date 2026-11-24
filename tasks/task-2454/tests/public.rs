use task_2454::*;

#[test]
fn public_examples() {
    assert_eq!(forward_raw_reads_peaks(&[]), vec![]);
    assert_eq!(forward_raw_reads_peaks(&[4]), vec![4u32]);
}
