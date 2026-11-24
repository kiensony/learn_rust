use task_2470::*;

#[test]
fn public_examples() {
    assert_eq!(reverse_raw_reads_reverse(&[]), vec![]);
    assert_eq!(reverse_raw_reads_reverse(&[4]), vec![4u32]);
}
