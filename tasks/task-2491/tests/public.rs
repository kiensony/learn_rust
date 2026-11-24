use task_2491::*;

#[test]
fn public_examples() {
    assert_eq!(interior_raw_reads_trace(&[]), vec![]);
    assert_eq!(interior_raw_reads_trace(&[4]), vec![]);
}
