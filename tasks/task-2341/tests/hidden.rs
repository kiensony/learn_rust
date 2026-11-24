use task_2341::*;

#[test]
fn edge_cases() {
    assert_eq!(
        baseline_drift_trace(&[9, 2, 2, 8, 0, 5]),
        vec![0u32, 7u32, 7u32, 1u32, 9u32, 4u32]
    );
    assert_eq!(baseline_drift_trace(&[0, 0, 0]), vec![0u32, 0u32, 0u32]);
    assert_eq!(
        baseline_drift_trace(&[255, 1, 255, 2]),
        vec![0u32, 254u32, 0u32, 253u32]
    );
    assert_eq!(
        baseline_drift_trace(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![
            0u32, 3u32, 6u32, 9u32, 18u32, 12u32, 3u32, 23u32, 5u32, 28u32
        ]
    );
    assert_eq!(
        baseline_drift_trace(&[17, 17, 1, 0, 2]),
        vec![0u32, 0u32, 16u32, 17u32, 15u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2341::TRACK_PROOF, 0x52540000 | 2341);
}
