use task_2455::*;

#[test]
fn public_examples() {
    assert_eq!(forward_raw_reads_changes(&[]), vec![]);
    assert_eq!(forward_raw_reads_changes(&[4]), vec![4u32]);
}
