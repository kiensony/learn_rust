use task_1946::*;

#[test]
fn public_examples() {
    assert_eq!(
        one_slot_queue_threshold_crossing(&[9, 2, 2, 8, 0, 5], 0),
        Some((0, 9))
    );
    assert_eq!(one_slot_queue_threshold_crossing(&[], 0), None);
}
