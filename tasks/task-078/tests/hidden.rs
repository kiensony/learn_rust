use task_078::*;

#[test]
fn edge_cases() {
    assert_eq!(two_counters_3(0, 6), (2, 0,));
    assert_eq!(two_counters_3(1, 9), (3, 1,));
    assert_eq!(two_counters_3(2, 12), (4, 2,));
    assert_eq!(two_counters_3(3, 15), (6, 0,));
    assert_eq!(two_counters_3(4, 1), (1, 2,));
    assert_eq!(two_counters_3(65535, 4), (21845, 0,));
    assert_eq!(two_counters_3(65535, 1), (21845, 0,));
    assert_eq!(two_counters_3(0, 0), (0, 0,));
    assert_eq!(two_counters_3(1, 0), (0, 1,));
    assert_eq!(two_counters_3(2, 0), (0, 2,));
    assert_eq!(two_counters_3(3, 0), (1, 0,));
    assert_eq!(two_counters_3(5, 0), (1, 2,));
    assert_eq!(two_counters_3(6, 0), (2, 0,));
    assert_eq!(two_counters_3(7, 0), (2, 1,));
    assert_eq!(two_counters_3(20, 0), (6, 2,));
    assert_eq!(two_counters_3(100, 0), (33, 1,));
}

#[test]
fn submission_marker() {
    assert_eq!(task_078::TRACK_PROOF, 0x52540000 | 78);
}
