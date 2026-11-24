use task_2072::*;

#[test]
fn edge_cases() {
    assert_eq!(three_slot_mailbox_total(&[9, 2, 2, 8, 0, 5]), 18);
    assert_eq!(three_slot_mailbox_total(&[0, 0, 0]), 0);
    assert_eq!(three_slot_mailbox_total(&[255, 1, 255, 2]), 511);
    assert_eq!(
        three_slot_mailbox_total(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        44
    );
    assert_eq!(three_slot_mailbox_total(&[17, 17, 1, 0, 2]), 37);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2072::TRACK_PROOF, 0x52540000 | 2072);
}
