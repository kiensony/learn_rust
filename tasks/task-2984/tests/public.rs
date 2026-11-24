use task_2984::*;

#[test]
fn public_examples() {
    assert_eq!(service_wait_peaks(&[]), vec![]);
    assert_eq!(service_wait_peaks(&[4]), vec![]);
}
