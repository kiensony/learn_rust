use task_1917::*;

#[test]
fn edge_cases() {
    assert_eq!(
        two_slot_queue_distinct(&[9, 2, 2, 8, 0, 5]),
        vec![2u32, 5u32, 9u32]
    );
    assert_eq!(two_slot_queue_distinct(&[0, 0, 0]), vec![]);
    assert_eq!(
        two_slot_queue_distinct(&[255, 1, 255, 2]),
        vec![1u32, 255u32]
    );
    assert_eq!(
        two_slot_queue_distinct(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![3u32, 6u32, 26u32]
    );
    assert_eq!(
        two_slot_queue_distinct(&[17, 17, 1, 0, 2]),
        vec![2u32, 17u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1917::TRACK_PROOF, 0x52540000 | 1917);
}
