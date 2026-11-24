use task_1241::*;

#[test]
fn public_examples() {
    assert_eq!(limited_reads_trace(&[]), vec![]);
    assert_eq!(limited_reads_trace(&[4]), vec![4u32]);
}
