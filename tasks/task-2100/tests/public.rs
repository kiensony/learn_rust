use task_2100::*;

#[test]
fn public_examples() {
    assert_eq!(
        five_slot_ingress_replay_mismatch(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], &[3, 6, 9, 12, 21]),
        Some(5)
    );
    assert_eq!(five_slot_ingress_replay_mismatch(&[], &[]), None);
}
