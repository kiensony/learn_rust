use task_2494::*;

#[test]
fn public_examples() {
    assert_eq!(interior_raw_reads_peaks(&[]), vec![]);
    assert_eq!(interior_raw_reads_peaks(&[4]), vec![]);
}
