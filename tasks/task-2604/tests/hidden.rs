use task_2604::*;

#[test]
fn edge_cases() {
    assert_eq!(embedded_checksum_peaks(&[9, 2, 2, 8, 0, 5]), vec![132u32]);
    assert_eq!(embedded_checksum_peaks(&[0, 0, 0]), vec![0u32]);
    assert_eq!(embedded_checksum_peaks(&[255, 1, 255, 2]), vec![255u32]);
    assert_eq!(
        embedded_checksum_peaks(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![129u32, 132u32, 138u32, 143u32]
    );
    assert_eq!(embedded_checksum_peaks(&[17, 17, 1, 0, 2]), vec![136u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2604::TRACK_PROOF, 0x52540000 | 2604);
}
