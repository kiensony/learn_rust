use task_2052::*;

#[test]
fn edge_cases() {
    assert_eq!(single_slot_receiver_total(&[9, 2, 2, 8, 0, 5]), 14);
    assert_eq!(single_slot_receiver_total(&[0, 0, 0]), 0);
    assert_eq!(single_slot_receiver_total(&[255, 1, 255, 2]), 255);
    assert_eq!(
        single_slot_receiver_total(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        29
    );
    assert_eq!(single_slot_receiver_total(&[17, 17, 1, 0, 2]), 19);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2052::TRACK_PROOF, 0x52540000 | 2052);
}
