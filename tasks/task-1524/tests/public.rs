use task_1524::*;

#[test]
fn public_examples() {
    assert_eq!(packet_parity_peaks(&[]), vec![]);
    assert_eq!(packet_parity_peaks(&[4]), vec![1u32]);
}
