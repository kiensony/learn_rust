use task_228::*;

#[test]
fn edge_cases() {
    assert_eq!(countdown_checksum_3(0, 6), 9);
    assert_eq!(countdown_checksum_3(1, 9), 22);
    assert_eq!(countdown_checksum_3(2, 12), 40);
    assert_eq!(countdown_checksum_3(3, 15), 63);
    assert_eq!(countdown_checksum_3(4, 1), 7);
    assert_eq!(countdown_checksum_3(255, 4), 10965);
    assert_eq!(countdown_checksum_3(255, 1), 10965);
    assert_eq!(countdown_checksum_3(0, 0), 0);
    assert_eq!(countdown_checksum_3(1, 0), 1);
    assert_eq!(countdown_checksum_3(2, 0), 2);
    assert_eq!(countdown_checksum_3(3, 0), 3);
    assert_eq!(countdown_checksum_3(5, 0), 7);
    assert_eq!(countdown_checksum_3(6, 0), 9);
    assert_eq!(countdown_checksum_3(7, 0), 12);
    assert_eq!(countdown_checksum_3(20, 0), 77);
    assert_eq!(countdown_checksum_3(100, 0), 1717);
}

#[test]
fn submission_marker() {
    assert_eq!(task_228::TRACK_PROOF, 0x52540000 | 228);
}
