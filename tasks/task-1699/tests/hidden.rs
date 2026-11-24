use task_1699::*;

#[test]
fn edge_cases() {
    assert_eq!(preview_version_frequent_readings("0.0.1", 0), vec![]);
    assert_eq!(
        preview_version_frequent_readings("1.0.0 1.2.0 2.0.0", 1),
        vec![0u32]
    );
    assert_eq!(
        preview_version_frequent_readings("1 3.1.8 bad 3.1.8", 2),
        vec![0u32]
    );
    assert_eq!(
        preview_version_frequent_readings("0.255.3 1.0.9 256.1.1", 3),
        vec![1u32, 0u32]
    );
    assert_eq!(
        preview_version_frequent_readings("12.4.8 2.9.3 12.4.8", 256),
        vec![0u32]
    );
    assert_eq!(
        preview_version_frequent_readings("1.0.0 1.2.0 2.0.0", 1),
        vec![0u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1699::TRACK_PROOF, 0x52540000 | 1699);
}
