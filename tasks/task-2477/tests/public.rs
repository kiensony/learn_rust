use task_2477::*;

#[test]
fn public_examples() {
    assert_eq!(strided_raw_reads_distinct(&[]), vec![]);
    assert_eq!(strided_raw_reads_distinct(&[4]), vec![4u32]);
}
