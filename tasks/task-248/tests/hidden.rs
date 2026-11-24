use task_248::*;

#[test]
fn edge_cases() {
    assert_eq!(countdown_checksum_8(0, 2), 2);
    assert_eq!(countdown_checksum_8(1, 3), 2);
    assert_eq!(countdown_checksum_8(7, 4), 3);
    assert_eq!(countdown_checksum_8(8, 5), 3);
    assert_eq!(countdown_checksum_8(9, 6), 3);
    assert_eq!(countdown_checksum_8(255, 0), 4192);
    assert_eq!(countdown_checksum_8(18, 3), 22);
    assert_eq!(countdown_checksum_8(3, 18), 22);
    assert_eq!(countdown_checksum_8(4, 4), 0);
    assert_eq!(countdown_checksum_8(0, 0), 0);
    assert_eq!(countdown_checksum_8(1, 0), 1);
    assert_eq!(countdown_checksum_8(7, 0), 7);
    assert_eq!(countdown_checksum_8(8, 0), 8);
    assert_eq!(countdown_checksum_8(9, 0), 10);
    assert_eq!(countdown_checksum_8(10, 0), 12);
    assert_eq!(countdown_checksum_8(11, 0), 14);
    assert_eq!(countdown_checksum_8(16, 0), 24);
    assert_eq!(countdown_checksum_8(17, 0), 27);
    assert_eq!(countdown_checksum_8(20, 0), 36);
    assert_eq!(countdown_checksum_8(100, 0), 676);
}

#[test]
fn submission_marker() {
    assert_eq!(task_248::TRACK_PROOF, 0x52540000 | 248);
}
