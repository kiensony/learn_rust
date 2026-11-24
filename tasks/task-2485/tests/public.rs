use task_2485::*;

#[test]
fn public_examples() {
    assert_eq!(trailing_raw_reads_changes(&[]), vec![]);
    assert_eq!(trailing_raw_reads_changes(&[4]), vec![4u32]);
}
