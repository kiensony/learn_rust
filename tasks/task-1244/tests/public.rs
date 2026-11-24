use task_1244::*;

#[test]
fn public_examples() {
    assert_eq!(limited_reads_peaks(&[]), vec![]);
    assert_eq!(limited_reads_peaks(&[4]), vec![4u32]);
}
