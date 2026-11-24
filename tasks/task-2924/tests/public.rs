use task_2924::*;

#[test]
fn public_examples() {
    assert_eq!(failure_threshold_peaks(&[]), vec![]);
    assert_eq!(failure_threshold_peaks(&[4]), vec![]);
}
