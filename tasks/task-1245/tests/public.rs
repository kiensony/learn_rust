use task_1245::*;

#[test]
fn public_examples() {
    assert_eq!(limited_reads_changes(&[]), vec![]);
    assert_eq!(limited_reads_changes(&[4]), vec![4u32]);
}
