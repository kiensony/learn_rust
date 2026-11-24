use task_2492::*;

#[test]
fn public_examples() {
    assert_eq!(interior_raw_reads_total(&[]), 0);
    assert_eq!(interior_raw_reads_total(&[4]), 0);
}
