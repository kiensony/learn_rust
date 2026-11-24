use task_093::*;

#[test]
fn edge_cases() {
    assert_eq!(complete_rows_8(0, 2), 0);
    assert_eq!(complete_rows_8(1, 3), 0);
    assert_eq!(complete_rows_8(7, 4), 0);
    assert_eq!(complete_rows_8(8, 5), 0);
    assert_eq!(complete_rows_8(9, 6), 0);
    assert_eq!(complete_rows_8(65535, 0), 8191);
    assert_eq!(complete_rows_8(18, 3), 1);
    assert_eq!(complete_rows_8(3, 18), 1);
    assert_eq!(complete_rows_8(4, 4), 0);
    assert_eq!(complete_rows_8(0, 0), 0);
    assert_eq!(complete_rows_8(1, 0), 0);
    assert_eq!(complete_rows_8(7, 0), 0);
    assert_eq!(complete_rows_8(8, 0), 1);
    assert_eq!(complete_rows_8(9, 0), 1);
    assert_eq!(complete_rows_8(10, 0), 1);
    assert_eq!(complete_rows_8(11, 0), 1);
    assert_eq!(complete_rows_8(16, 0), 2);
    assert_eq!(complete_rows_8(17, 0), 2);
    assert_eq!(complete_rows_8(20, 0), 2);
    assert_eq!(complete_rows_8(100, 0), 12);
}

#[test]
fn submission_marker() {
    assert_eq!(task_093::TRACK_PROOF, 0x52540000 | 93);
}
