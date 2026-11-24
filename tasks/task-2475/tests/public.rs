use task_2475::*;

#[test]
fn public_examples() {
    assert_eq!(strided_raw_reads_changes(&[]), vec![]);
    assert_eq!(strided_raw_reads_changes(&[4]), vec![4u32]);
}
