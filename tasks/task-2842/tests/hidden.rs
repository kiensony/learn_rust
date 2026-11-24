use task_2842::*;

#[test]
fn edge_cases() {
    assert_eq!(service_status_total(&[9, 2, 2, 8, 0, 5]), 0);
    assert_eq!(service_status_total(&[0, 0, 0]), 0);
    assert_eq!(service_status_total(&[255, 1, 255, 2]), 2);
    assert_eq!(
        service_status_total(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        0
    );
    assert_eq!(service_status_total(&[17, 17, 1, 0, 2]), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2842::TRACK_PROOF, 0x52540000 | 2842);
}
