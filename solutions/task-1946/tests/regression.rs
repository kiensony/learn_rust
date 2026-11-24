use task_1946::*;

#[test]
fn transfer_case() {
    assert_eq!(
        one_slot_queue_threshold_crossing(&[9, 2, 2, 8, 0, 5, 5, 0, 8, 2, 2, 9], 1),
        Some((0, 9))
    );
}
