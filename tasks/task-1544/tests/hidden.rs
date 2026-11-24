use task_1544::*;

#[test]
fn edge_cases() {
    assert_eq!(
        packet_reversal_peaks(&[9, 2, 2, 8, 0, 5]),
        vec![144u32, 160u32]
    );
    assert_eq!(packet_reversal_peaks(&[0, 0, 0]), vec![0u32]);
    assert_eq!(packet_reversal_peaks(&[255, 1, 255, 2]), vec![255u32]);
    assert_eq!(
        packet_reversal_peaks(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![192u32, 240u32, 248u32]
    );
    assert_eq!(packet_reversal_peaks(&[17, 17, 1, 0, 2]), vec![136u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1544::TRACK_PROOF, 0x52540000 | 1544);
}
