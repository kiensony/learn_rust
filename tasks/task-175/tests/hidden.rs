use task_175::*;

#[test]
fn edge_cases() {
    assert_eq!(service_interval_3(13, 3), false);
    assert_eq!(service_interval_3(1, 9), false);
    assert_eq!(service_interval_3(2, 12), false);
    assert_eq!(service_interval_3(3, 15), true);
    assert_eq!(service_interval_3(4, 1), false);
    assert_eq!(service_interval_3(65535, 4), true);
    assert_eq!(service_interval_3(5, 7), true);
    assert_eq!(service_interval_3(6, 10), false);
    assert_eq!(service_interval_3(6, 13), false);
    assert_eq!(service_interval_3(7, 16), false);
    assert_eq!(service_interval_3(65535, 1), true);
    assert_eq!(service_interval_3(0, 0), false);
    assert_eq!(service_interval_3(1, 0), false);
    assert_eq!(service_interval_3(2, 0), false);
    assert_eq!(service_interval_3(3, 0), true);
    assert_eq!(service_interval_3(5, 0), false);
    assert_eq!(service_interval_3(6, 0), true);
    assert_eq!(service_interval_3(7, 0), false);
    assert_eq!(service_interval_3(20, 0), false);
    assert_eq!(service_interval_3(100, 0), false);
}

#[test]
fn submission_marker() {
    assert_eq!(task_175::TRACK_PROOF, 0x52540000 | 175);
}
