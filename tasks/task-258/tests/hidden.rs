use task_258::*;

#[test]
fn edge_cases() {
    assert_eq!(countdown_checksum_10(255, 0), 0);
    assert_eq!(countdown_checksum_10(1, 2), 1);
    assert_eq!(countdown_checksum_10(255, 9), 9);
    assert_eq!(countdown_checksum_10(10, 11), 10);
    assert_eq!(countdown_checksum_10(255, 11), 12);
    assert_eq!(countdown_checksum_10(255, 255), 3380);
    assert_eq!(countdown_checksum_10(1, 0), 0);
    assert_eq!(countdown_checksum_10(0, 1), 0);
    assert_eq!(countdown_checksum_10(0, 0), 0);
    assert_eq!(countdown_checksum_10(1, 1), 1);
    assert_eq!(countdown_checksum_10(9, 9), 9);
    assert_eq!(countdown_checksum_10(10, 10), 10);
    assert_eq!(countdown_checksum_10(11, 11), 12);
    assert_eq!(countdown_checksum_10(12, 12), 14);
    assert_eq!(countdown_checksum_10(13, 13), 16);
    assert_eq!(countdown_checksum_10(20, 20), 30);
    assert_eq!(countdown_checksum_10(21, 21), 33);
}

#[test]
fn submission_marker() {
    assert_eq!(task_258::TRACK_PROOF, 0x52540000 | 258);
}
