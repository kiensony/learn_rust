use task_2495::*;

#[test]
fn public_examples() {
    assert_eq!(interior_raw_reads_changes(&[]), vec![]);
    assert_eq!(interior_raw_reads_changes(&[4]), vec![]);
}
