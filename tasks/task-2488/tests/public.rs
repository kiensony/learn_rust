use task_2488::*;

#[test]
fn public_examples() {
    assert_eq!(trailing_raw_reads_steps(&[]), vec![]);
    assert_eq!(trailing_raw_reads_steps(&[4]), vec![]);
}
