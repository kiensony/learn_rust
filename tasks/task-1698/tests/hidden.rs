use task_1698::*;

#[test]
fn edge_cases() {
    assert_eq!(patch_version_rolling_load("0.0.1", 1), vec![1u64]);
    assert_eq!(
        patch_version_rolling_load("1.0.0 1.2.0 2.0.0", 2),
        vec![0u64, 0u64]
    );
    assert_eq!(patch_version_rolling_load("1 3.1.8 bad 3.1.8", 0), vec![]);
    assert_eq!(
        patch_version_rolling_load("0.255.3 1.0.9 256.1.1", 100),
        vec![]
    );
    assert_eq!(
        patch_version_rolling_load("12.4.8 2.9.3 12.4.8", 3),
        vec![19u64]
    );
    assert_eq!(
        patch_version_rolling_load("12.4.8 2.9.3 12.4.8", 2),
        vec![11u64, 11u64]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1698::TRACK_PROOF, 0x52540000 | 1698);
}
