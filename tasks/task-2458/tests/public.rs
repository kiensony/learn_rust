use task_2458::*;

#[test]
fn public_examples() {
    assert_eq!(forward_raw_reads_steps(&[]), vec![]);
    assert_eq!(forward_raw_reads_steps(&[4]), vec![]);
}
