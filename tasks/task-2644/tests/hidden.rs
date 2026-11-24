use task_2644::*;

#[test]
fn edge_cases() {
    assert_eq!(
        embedded_nibble_swap_peaks(&[9, 2, 2, 8, 0, 5]),
        vec![144u32]
    );
    assert_eq!(embedded_nibble_swap_peaks(&[0, 0, 0]), vec![0u32]);
    assert_eq!(embedded_nibble_swap_peaks(&[255, 1, 255, 2]), vec![255u32]);
    assert_eq!(
        embedded_nibble_swap_peaks(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![48u32, 96u32, 144u32, 192u32, 240u32, 241u32]
    );
    assert_eq!(
        embedded_nibble_swap_peaks(&[17, 17, 1, 0, 2]),
        vec![17u32, 32u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2644::TRACK_PROOF, 0x52540000 | 2644);
}
