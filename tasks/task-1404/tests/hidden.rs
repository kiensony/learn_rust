use task_1404::*;

#[test]
fn edge_cases() {
    assert_eq!(
        millisecond_remainder_peaks(&[9, 2, 2, 8, 0, 5]),
        vec![153u32]
    );
    assert_eq!(millisecond_remainder_peaks(&[0, 0, 0]), vec![0u32]);
    assert_eq!(millisecond_remainder_peaks(&[255, 1, 255, 2]), vec![335u32]);
    assert_eq!(
        millisecond_remainder_peaks(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![51u32, 102u32, 153u32, 204u32, 357u32, 442u32, 527u32]
    );
    assert_eq!(
        millisecond_remainder_peaks(&[17, 17, 1, 0, 2]),
        vec![289u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1404::TRACK_PROOF, 0x52540000 | 1404);
}
