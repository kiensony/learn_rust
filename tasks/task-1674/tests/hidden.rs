use task_1674::*;

#[test]
fn edge_cases() {
    assert_eq!(patch_version_peaks("1.0.0 1.2.0 2.0.0"), vec![0u32]);
    assert_eq!(patch_version_peaks("1 3.1.8 bad 3.1.8"), vec![8u32]);
    assert_eq!(
        patch_version_peaks("0.255.3 1.0.9 256.1.1"),
        vec![3u32, 9u32]
    );
    assert_eq!(patch_version_peaks("12.4.8 2.9.3 12.4.8"), vec![8u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1674::TRACK_PROOF, 0x52540000 | 1674);
}
