use task_2685::*;

#[test]
fn edge_cases() {
    assert_eq!(
        universal_slice_sum_changes(&[9, 2, 2, 8, 0, 5]),
        vec![21u32, 5u32]
    );
    assert_eq!(universal_slice_sum_changes(&[0, 0, 0]), vec![0u32]);
    assert_eq!(universal_slice_sum_changes(&[255, 1, 255, 2]), vec![513u32]);
    assert_eq!(
        universal_slice_sum_changes(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![51u32, 80u32]
    );
    assert_eq!(universal_slice_sum_changes(&[17, 17, 1, 0, 2]), vec![37u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2685::TRACK_PROOF, 0x52540000 | 2685);
}
