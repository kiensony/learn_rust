use task_098::*;

#[test]
fn edge_cases() {
    assert_eq!(two_counters_8(0, 2), (0, 2,));
    assert_eq!(two_counters_8(1, 3), (0, 2,));
    assert_eq!(two_counters_8(7, 4), (0, 3,));
    assert_eq!(two_counters_8(8, 5), (0, 3,));
    assert_eq!(two_counters_8(9, 6), (0, 3,));
    assert_eq!(two_counters_8(65535, 0), (8191, 7,));
    assert_eq!(two_counters_8(18, 3), (1, 7,));
    assert_eq!(two_counters_8(3, 18), (1, 7,));
    assert_eq!(two_counters_8(4, 4), (0, 0,));
    assert_eq!(two_counters_8(0, 0), (0, 0,));
    assert_eq!(two_counters_8(1, 0), (0, 1,));
    assert_eq!(two_counters_8(7, 0), (0, 7,));
    assert_eq!(two_counters_8(8, 0), (1, 0,));
    assert_eq!(two_counters_8(9, 0), (1, 1,));
    assert_eq!(two_counters_8(10, 0), (1, 2,));
    assert_eq!(two_counters_8(11, 0), (1, 3,));
    assert_eq!(two_counters_8(16, 0), (2, 0,));
    assert_eq!(two_counters_8(17, 0), (2, 1,));
    assert_eq!(two_counters_8(20, 0), (2, 4,));
    assert_eq!(two_counters_8(100, 0), (12, 4,));
}

#[test]
fn submission_marker() {
    assert_eq!(task_098::TRACK_PROOF, 0x52540000 | 98);
}
