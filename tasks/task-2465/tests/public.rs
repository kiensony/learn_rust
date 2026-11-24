use task_2465::*;

#[test]
fn public_examples() {
    assert_eq!(reverse_raw_reads_changes(&[]), vec![]);
    assert_eq!(reverse_raw_reads_changes(&[4]), vec![4u32]);
}
