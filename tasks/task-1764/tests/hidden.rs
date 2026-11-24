use task_1764::*;

#[test]
fn edge_cases() {
    assert_eq!(generated_offset_peaks(&[9, 2, 2, 8, 0, 5]), vec![16u32]);
    assert_eq!(generated_offset_peaks(&[0, 0, 0]), vec![7u32]);
    assert_eq!(generated_offset_peaks(&[255, 1, 255, 2]), vec![262u32]);
    assert_eq!(
        generated_offset_peaks(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![10u32, 13u32, 16u32, 19u32, 28u32, 33u32, 38u32]
    );
    assert_eq!(generated_offset_peaks(&[17, 17, 1, 0, 2]), vec![24u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1764::TRACK_PROOF, 0x52540000 | 1764);
}
