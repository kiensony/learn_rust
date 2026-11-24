use task_083::*;

#[test]
fn edge_cases() {
    assert_eq!(complete_rows_5(0, 2), 0);
    assert_eq!(complete_rows_5(1, 0), 0);
    assert_eq!(complete_rows_5(4, 1), 0);
    assert_eq!(complete_rows_5(5, 2), 0);
    assert_eq!(complete_rows_5(6, 0), 1);
    assert_eq!(complete_rows_5(65535, 1), 13106);
    assert_eq!(complete_rows_5(0, 9), 0);
    assert_eq!(complete_rows_5(100, 0), 20);
    assert_eq!(complete_rows_5(100, 1), 19);
    assert_eq!(complete_rows_5(0, 0), 0);
    assert_eq!(complete_rows_5(5, 0), 1);
    assert_eq!(complete_rows_5(7, 0), 1);
    assert_eq!(complete_rows_5(8, 0), 1);
    assert_eq!(complete_rows_5(10, 0), 2);
    assert_eq!(complete_rows_5(11, 0), 2);
    assert_eq!(complete_rows_5(20, 0), 4);
}

#[test]
fn submission_marker() {
    assert_eq!(task_083::TRACK_PROOF, 0x52540000 | 83);
}
