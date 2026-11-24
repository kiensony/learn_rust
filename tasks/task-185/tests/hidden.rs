use task_185::*;

#[test]
fn edge_cases() {
    assert_eq!(service_interval_5(13, 1), false);
    assert_eq!(service_interval_5(0, 2), false);
    assert_eq!(service_interval_5(1, 0), false);
    assert_eq!(service_interval_5(4, 1), false);
    assert_eq!(service_interval_5(5, 2), false);
    assert_eq!(service_interval_5(6, 0), false);
    assert_eq!(service_interval_5(65535, 1), false);
    assert_eq!(service_interval_5(8, 0), false);
    assert_eq!(service_interval_5(10, 1), false);
    assert_eq!(service_interval_5(11, 2), false);
    assert_eq!(service_interval_5(0, 9), false);
    assert_eq!(service_interval_5(100, 0), true);
    assert_eq!(service_interval_5(100, 1), false);
    assert_eq!(service_interval_5(0, 0), false);
    assert_eq!(service_interval_5(5, 0), true);
    assert_eq!(service_interval_5(7, 0), false);
    assert_eq!(service_interval_5(10, 0), true);
    assert_eq!(service_interval_5(11, 0), false);
    assert_eq!(service_interval_5(20, 0), true);
}

#[test]
fn submission_marker() {
    assert_eq!(task_185::TRACK_PROOF, 0x52540000 | 185);
}
