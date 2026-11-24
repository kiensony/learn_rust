use task_238::*;

#[test]
fn edge_cases() {
    assert_eq!(countdown_checksum_5(0, 2), 0);
    assert_eq!(countdown_checksum_5(1, 0), 1);
    assert_eq!(countdown_checksum_5(4, 1), 3);
    assert_eq!(countdown_checksum_5(5, 2), 3);
    assert_eq!(countdown_checksum_5(6, 0), 7);
    assert_eq!(countdown_checksum_5(255, 1), 6579);
    assert_eq!(countdown_checksum_5(0, 9), 0);
    assert_eq!(countdown_checksum_5(100, 0), 1050);
    assert_eq!(countdown_checksum_5(100, 1), 1030);
    assert_eq!(countdown_checksum_5(0, 0), 0);
    assert_eq!(countdown_checksum_5(5, 0), 5);
    assert_eq!(countdown_checksum_5(7, 0), 9);
    assert_eq!(countdown_checksum_5(8, 0), 11);
    assert_eq!(countdown_checksum_5(10, 0), 15);
    assert_eq!(countdown_checksum_5(11, 0), 18);
    assert_eq!(countdown_checksum_5(20, 0), 50);
}

#[test]
fn submission_marker() {
    assert_eq!(task_238::TRACK_PROOF, 0x52540000 | 238);
}
