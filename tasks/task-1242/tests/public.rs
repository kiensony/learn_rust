use task_1242::*;

#[test]
fn public_examples() {
    assert_eq!(limited_reads_total(&[]), 0);
    assert_eq!(limited_reads_total(&[4]), 4);
}
