use task_2493::*;

#[test]
fn public_examples() {
    assert_eq!(interior_raw_reads_bounds(&[]), None);
    assert_eq!(interior_raw_reads_bounds(&[4]), None);
}
