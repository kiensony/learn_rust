use task_1504::*;

#[test]
fn public_examples() {
    assert_eq!(packet_high_nibble_peaks(&[]), vec![]);
    assert_eq!(packet_high_nibble_peaks(&[4]), vec![0u32]);
}
