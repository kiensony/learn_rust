use task_103::*;

#[test]
fn edge_cases() {
    assert_eq!(complete_rows_10(65535, 0), 0);
    assert_eq!(complete_rows_10(1, 2), 0);
    assert_eq!(complete_rows_10(65535, 9), 0);
    assert_eq!(complete_rows_10(10, 11), 1);
    assert_eq!(complete_rows_10(65535, 11), 1);
    assert_eq!(complete_rows_10(65535, 65535), 6553);
    assert_eq!(complete_rows_10(1, 0), 0);
    assert_eq!(complete_rows_10(0, 1), 0);
    assert_eq!(complete_rows_10(0, 0), 0);
    assert_eq!(complete_rows_10(1, 1), 0);
    assert_eq!(complete_rows_10(9, 9), 0);
    assert_eq!(complete_rows_10(10, 10), 1);
    assert_eq!(complete_rows_10(11, 11), 1);
    assert_eq!(complete_rows_10(12, 12), 1);
    assert_eq!(complete_rows_10(13, 13), 1);
    assert_eq!(complete_rows_10(20, 20), 2);
    assert_eq!(complete_rows_10(21, 21), 2);
    assert_eq!(complete_rows_10(100, 100), 10);
}

#[test]
fn submission_marker() {
    assert_eq!(task_103::TRACK_PROOF, 0x52540000 | 103);
}
