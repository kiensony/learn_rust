use task_2487::*;

#[test]
fn public_examples() {
    assert_eq!(trailing_raw_reads_distinct(&[]), vec![]);
    assert_eq!(trailing_raw_reads_distinct(&[4]), vec![4u32]);
}
