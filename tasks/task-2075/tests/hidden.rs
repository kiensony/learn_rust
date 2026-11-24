use task_2075::*;

#[test]
fn edge_cases() {
    assert_eq!(
        three_slot_mailbox_changes(&[9, 2, 2, 8, 0, 5]),
        vec![9u32, 2u32, 5u32]
    );
    assert_eq!(three_slot_mailbox_changes(&[0, 0, 0]), vec![]);
    assert_eq!(
        three_slot_mailbox_changes(&[255, 1, 255, 2]),
        vec![255u32, 1u32, 255u32]
    );
    assert_eq!(
        three_slot_mailbox_changes(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![3u32, 6u32, 9u32, 26u32]
    );
    assert_eq!(
        three_slot_mailbox_changes(&[17, 17, 1, 0, 2]),
        vec![17u32, 1u32, 2u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2075::TRACK_PROOF, 0x52540000 | 2075);
}
