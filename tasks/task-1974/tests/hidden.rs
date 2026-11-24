use task_1974::*;

#[test]
fn edge_cases() {
    assert_eq!(
        atomic_toggle_peaks(&[9, 2, 2, 8, 0, 5]),
        vec![0u32, 9u32, 11u32]
    );
    assert_eq!(atomic_toggle_peaks(&[0, 0, 0]), vec![0u32]);
    assert_eq!(atomic_toggle_peaks(&[255, 1, 255, 2]), vec![0u32, 255u32]);
    assert_eq!(
        atomic_toggle_peaks(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![0u32, 3u32, 5u32, 12u32, 21u32, 26u32]
    );
    assert_eq!(atomic_toggle_peaks(&[17, 17, 1, 0, 2]), vec![0u32, 17u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1974::TRACK_PROOF, 0x52540000 | 1974);
}
