use task_2460::*;

#[test]
fn public_examples() {
    assert_eq!(forward_raw_reads_reverse(&[]), vec![]);
    assert_eq!(forward_raw_reads_reverse(&[4]), vec![4u32]);
}
