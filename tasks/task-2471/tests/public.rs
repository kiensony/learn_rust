use task_2471::*;

#[test]
fn public_examples() {
    assert_eq!(strided_raw_reads_trace(&[]), vec![]);
    assert_eq!(strided_raw_reads_trace(&[4]), vec![4u32]);
}
