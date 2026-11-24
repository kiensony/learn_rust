use task_108::*;

#[test]
fn edge_cases() {
    assert_eq!(two_counters_10(65535, 0), (0, 0,));
    assert_eq!(two_counters_10(1, 2), (0, 1,));
    assert_eq!(two_counters_10(65535, 9), (0, 9,));
    assert_eq!(two_counters_10(10, 11), (1, 0,));
    assert_eq!(two_counters_10(65535, 11), (1, 1,));
    assert_eq!(two_counters_10(65535, 65535), (6553, 5,));
    assert_eq!(two_counters_10(1, 0), (0, 0,));
    assert_eq!(two_counters_10(0, 1), (0, 0,));
    assert_eq!(two_counters_10(0, 0), (0, 0,));
    assert_eq!(two_counters_10(1, 1), (0, 1,));
    assert_eq!(two_counters_10(9, 9), (0, 9,));
    assert_eq!(two_counters_10(10, 10), (1, 0,));
    assert_eq!(two_counters_10(11, 11), (1, 1,));
    assert_eq!(two_counters_10(12, 12), (1, 2,));
    assert_eq!(two_counters_10(13, 13), (1, 3,));
    assert_eq!(two_counters_10(20, 20), (2, 0,));
    assert_eq!(two_counters_10(21, 21), (2, 1,));
}

#[test]
fn submission_marker() {
    assert_eq!(task_108::TRACK_PROOF, 0x52540000 | 108);
}
