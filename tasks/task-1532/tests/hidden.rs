use task_1532::*;

#[test]
fn edge_cases() {
    assert_eq!(packet_gray_code_total(&[9, 2, 2, 8, 0, 5]), 38);
    assert_eq!(packet_gray_code_total(&[0, 0, 0]), 0);
    assert_eq!(packet_gray_code_total(&[255, 1, 255, 2]), 260);
    assert_eq!(
        packet_gray_code_total(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        120
    );
    assert_eq!(packet_gray_code_total(&[17, 17, 1, 0, 2]), 54);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1532::TRACK_PROOF, 0x52540000 | 1532);
}
