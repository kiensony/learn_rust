use task_1544::*;

#[test]
fn public_examples() {
    assert_eq!(packet_reversal_peaks(&[]), vec![]);
    assert_eq!(packet_reversal_peaks(&[4]), vec![32u32]);
}
