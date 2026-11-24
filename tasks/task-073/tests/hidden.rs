use task_073::*;

#[test]
fn edge_cases() {
    assert_eq!(complete_rows_3(0, 6), 2);
    assert_eq!(complete_rows_3(1, 9), 3);
    assert_eq!(complete_rows_3(2, 12), 4);
    assert_eq!(complete_rows_3(3, 15), 6);
    assert_eq!(complete_rows_3(4, 1), 1);
    assert_eq!(complete_rows_3(65535, 4), 21845);
    assert_eq!(complete_rows_3(65535, 1), 21845);
    assert_eq!(complete_rows_3(0, 0), 0);
    assert_eq!(complete_rows_3(1, 0), 0);
    assert_eq!(complete_rows_3(2, 0), 0);
    assert_eq!(complete_rows_3(3, 0), 1);
    assert_eq!(complete_rows_3(5, 0), 1);
    assert_eq!(complete_rows_3(6, 0), 2);
    assert_eq!(complete_rows_3(7, 0), 2);
    assert_eq!(complete_rows_3(20, 0), 6);
    assert_eq!(complete_rows_3(100, 0), 33);
}

#[test]
fn submission_marker() {
    assert_eq!(task_073::TRACK_PROOF, 0x52540000 | 73);
}
