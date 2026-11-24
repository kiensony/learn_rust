use task_2824::*;

#[test]
fn edge_cases() {
    assert_eq!(
        service_sharding_peaks(&[9, 2, 2, 8, 0, 5]),
        vec![1u32, 2u32]
    );
    assert_eq!(service_sharding_peaks(&[0, 0, 0]), vec![0u32]);
    assert_eq!(service_sharding_peaks(&[255, 1, 255, 2]), vec![3u32]);
    assert_eq!(
        service_sharding_peaks(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![3u32]
    );
    assert_eq!(service_sharding_peaks(&[17, 17, 1, 0, 2]), vec![1u32, 2u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2824::TRACK_PROOF, 0x52540000 | 2824);
}
