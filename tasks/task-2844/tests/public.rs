use task_2844::*;

#[test]
fn public_examples() {
    assert_eq!(service_status_peaks(&[]), vec![]);
    assert_eq!(service_status_peaks(&[4]), vec![0u32]);
}
