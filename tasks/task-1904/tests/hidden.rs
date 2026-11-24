use task_1904::*;

#[test]
fn edge_cases() {
    assert_eq!(one_slot_queue_peaks(&[9, 2, 2, 8, 0, 5]), vec![9u32]);
    assert_eq!(one_slot_queue_peaks(&[0, 0, 0]), vec![]);
    assert_eq!(one_slot_queue_peaks(&[255, 1, 255, 2]), vec![255u32]);
    assert_eq!(
        one_slot_queue_peaks(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![3u32, 26u32]
    );
    assert_eq!(one_slot_queue_peaks(&[17, 17, 1, 0, 2]), vec![17u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1904::TRACK_PROOF, 0x52540000 | 1904);
}
