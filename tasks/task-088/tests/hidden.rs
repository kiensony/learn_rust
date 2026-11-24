use task_088::*;

#[test]
fn edge_cases() {
    assert_eq!(two_counters_5(0, 2), (0, 0,));
    assert_eq!(two_counters_5(1, 0), (0, 1,));
    assert_eq!(two_counters_5(4, 1), (0, 3,));
    assert_eq!(two_counters_5(5, 2), (0, 3,));
    assert_eq!(two_counters_5(6, 0), (1, 1,));
    assert_eq!(two_counters_5(65535, 1), (13106, 4,));
    assert_eq!(two_counters_5(0, 9), (0, 0,));
    assert_eq!(two_counters_5(100, 0), (20, 0,));
    assert_eq!(two_counters_5(100, 1), (19, 4,));
    assert_eq!(two_counters_5(0, 0), (0, 0,));
    assert_eq!(two_counters_5(5, 0), (1, 0,));
    assert_eq!(two_counters_5(7, 0), (1, 2,));
    assert_eq!(two_counters_5(8, 0), (1, 3,));
    assert_eq!(two_counters_5(10, 0), (2, 0,));
    assert_eq!(two_counters_5(11, 0), (2, 1,));
    assert_eq!(two_counters_5(20, 0), (4, 0,));
}

#[test]
fn submission_marker() {
    assert_eq!(task_088::TRACK_PROOF, 0x52540000 | 88);
}
