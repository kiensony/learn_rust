use task_2451::*;

#[test]
fn public_examples() {
    assert_eq!(forward_raw_reads_trace(&[]), vec![]);
    assert_eq!(forward_raw_reads_trace(&[4]), vec![4u32]);
}
