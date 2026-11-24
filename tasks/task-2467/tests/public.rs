use task_2467::*;

#[test]
fn public_examples() {
    assert_eq!(reverse_raw_reads_distinct(&[]), vec![]);
    assert_eq!(reverse_raw_reads_distinct(&[4]), vec![4u32]);
}
