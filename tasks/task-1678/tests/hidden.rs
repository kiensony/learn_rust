use task_1678::*;

#[test]
fn edge_cases() {
    assert_eq!(patch_version_steps("1.0.0 1.2.0 2.0.0"), vec![0u32, 0u32]);
    assert_eq!(patch_version_steps("1 3.1.8 bad 3.1.8"), vec![0u32]);
    assert_eq!(patch_version_steps("0.255.3 1.0.9 256.1.1"), vec![6u32]);
    assert_eq!(patch_version_steps("12.4.8 2.9.3 12.4.8"), vec![5u32, 5u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1678::TRACK_PROOF, 0x52540000 | 1678);
}
