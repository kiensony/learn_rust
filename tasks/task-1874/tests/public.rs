use task_1874::*;

#[test]
fn public_examples() {
    assert_eq!(shared_payload_peaks(&[]), vec![]);
    assert_eq!(shared_payload_peaks(&[4]), vec![4u32]);
}
