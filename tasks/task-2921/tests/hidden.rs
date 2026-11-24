use task_2921::*;

#[test]
fn edge_cases() {
    assert_eq!(
        failure_threshold_trace(&[9, 2, 2, 8, 0, 5]),
        vec![0u32, 0u32, 0u32]
    );
    assert_eq!(failure_threshold_trace(&[0, 0, 0]), vec![0u32]);
    assert_eq!(failure_threshold_trace(&[255, 1, 255, 2]), vec![0u32, 0u32]);
    assert_eq!(
        failure_threshold_trace(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![0u32, 0u32, 0u32, 0u32, 0u32]
    );
    assert_eq!(
        failure_threshold_trace(&[17, 17, 1, 0, 2]),
        vec![0u32, 0u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2921::TRACK_PROOF, 0x52540000 | 2921);
}
