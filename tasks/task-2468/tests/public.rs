use task_2468::*;

#[test]
fn public_examples() {
    assert_eq!(reverse_raw_reads_steps(&[]), vec![]);
    assert_eq!(reverse_raw_reads_steps(&[4]), vec![]);
}
