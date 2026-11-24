use task_2461::*;

#[test]
fn public_examples() {
    assert_eq!(reverse_raw_reads_trace(&[]), vec![]);
    assert_eq!(reverse_raw_reads_trace(&[4]), vec![4u32]);
}
