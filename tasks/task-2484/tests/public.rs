use task_2484::*;

#[test]
fn public_examples() {
    assert_eq!(trailing_raw_reads_peaks(&[]), vec![]);
    assert_eq!(trailing_raw_reads_peaks(&[4]), vec![4u32]);
}
