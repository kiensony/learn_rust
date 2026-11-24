use task_1922::*;

#[test]
fn edge_cases() {
    assert_eq!(four_slot_queue_total(&[9, 2, 2, 8, 0, 5]), 26);
    assert_eq!(four_slot_queue_total(&[0, 0, 0]), 0);
    assert_eq!(four_slot_queue_total(&[255, 1, 255, 2]), 513);
    assert_eq!(
        four_slot_queue_total(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        56
    );
    assert_eq!(four_slot_queue_total(&[17, 17, 1, 0, 2]), 37);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1922::TRACK_PROOF, 0x52540000 | 1922);
}
