use task_2481::*;

#[test]
fn public_examples() {
    assert_eq!(trailing_raw_reads_trace(&[]), vec![]);
    assert_eq!(trailing_raw_reads_trace(&[4]), vec![4u32]);
}
