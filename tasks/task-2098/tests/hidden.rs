use task_2098::*;

#[test]
fn edge_cases() {
    assert_eq!(three_slot_mailbox_rolling_load(&[4], 1), vec![4u64]);
    assert_eq!(
        three_slot_mailbox_rolling_load(&[9, 2, 2, 8, 0, 5], 2),
        vec![11u64, 4u64, 7u64]
    );
    assert_eq!(three_slot_mailbox_rolling_load(&[0, 0, 0], 0), vec![]);
    assert_eq!(
        three_slot_mailbox_rolling_load(&[255, 1, 255, 2], 100),
        vec![]
    );
    assert_eq!(
        three_slot_mailbox_rolling_load(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 3),
        vec![18u64, 41u64]
    );
    assert_eq!(
        three_slot_mailbox_rolling_load(&[17, 17, 1, 0, 2], 1),
        vec![17u64, 17u64, 1u64, 2u64]
    );
    assert_eq!(
        three_slot_mailbox_rolling_load(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 2),
        vec![9u64, 15u64, 35u64]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2098::TRACK_PROOF, 0x52540000 | 2098);
}
