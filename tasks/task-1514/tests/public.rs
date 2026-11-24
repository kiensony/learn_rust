use task_1514::*;

#[test]
fn public_examples() {
    assert_eq!(packet_low_nibble_peaks(&[]), vec![]);
    assert_eq!(packet_low_nibble_peaks(&[4]), vec![4u32]);
}
