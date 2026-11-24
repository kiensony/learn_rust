use task_068::*;

#[test]
fn edge_cases() {
    assert_eq!(two_counters_2(0), (0, 0,));
    assert_eq!(two_counters_2(1), (0, 1,));
    assert_eq!(two_counters_2(1), (0, 1,));
    assert_eq!(two_counters_2(2), (1, 0,));
    assert_eq!(two_counters_2(3), (1, 1,));
    assert_eq!(two_counters_2(65535), (32767, 1,));
}

#[test]
fn submission_marker() {
    assert_eq!(task_068::TRACK_PROOF, 0x52540000 | 68);
}
