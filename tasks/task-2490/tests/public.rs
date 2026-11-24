use task_2490::*;

#[test]
fn public_examples() {
    assert_eq!(trailing_raw_reads_reverse(&[]), vec![]);
    assert_eq!(trailing_raw_reads_reverse(&[4]), vec![4u32]);
}
