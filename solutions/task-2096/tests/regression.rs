use task_2096::*;

#[test]
fn transfer_case() {
    assert_eq!(
        single_slot_receiver_threshold_crossing(&[9, 2, 2, 8, 0, 5, 5, 0, 8, 2, 2, 9], 1),
        Some((0, 9))
    );
}
