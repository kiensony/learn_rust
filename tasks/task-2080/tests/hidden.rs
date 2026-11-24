use task_2080::*;

#[test]
fn edge_cases() {
    assert_eq!(
        three_slot_mailbox_reverse(&[9, 2, 2, 8, 0, 5]),
        vec![5u32, 2u32, 2u32, 9u32]
    );
    assert_eq!(three_slot_mailbox_reverse(&[0, 0, 0]), vec![]);
    assert_eq!(
        three_slot_mailbox_reverse(&[255, 1, 255, 2]),
        vec![255u32, 1u32, 255u32]
    );
    assert_eq!(
        three_slot_mailbox_reverse(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![26u32, 9u32, 6u32, 3u32]
    );
    assert_eq!(
        three_slot_mailbox_reverse(&[17, 17, 1, 0, 2]),
        vec![2u32, 1u32, 17u32, 17u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2080::TRACK_PROOF, 0x52540000 | 2080);
}
