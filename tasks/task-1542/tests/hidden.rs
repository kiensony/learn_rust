use task_1542::*;

#[test]
fn edge_cases() {
    assert_eq!(packet_reversal_total(&[9, 2, 2, 8, 0, 5]), 448);
    assert_eq!(packet_reversal_total(&[0, 0, 0]), 0);
    assert_eq!(packet_reversal_total(&[255, 1, 255, 2]), 702);
    assert_eq!(
        packet_reversal_total(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        1240
    );
    assert_eq!(packet_reversal_total(&[17, 17, 1, 0, 2]), 464);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1542::TRACK_PROOF, 0x52540000 | 1542);
}
